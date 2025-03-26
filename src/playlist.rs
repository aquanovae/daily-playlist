use crate::{ AppError, AppResult, Spotify };

use futures::{
    future,
    stream::StreamExt,
};

use rand::seq::SliceRandom;

use rspotify::{
    clients::{ BaseClient, OAuthClient },
    model::{ PlayableId, PlaylistId },
};

use std::{
    collections::HashMap,
    future::IntoFuture,
    iter::IntoIterator,
    sync::mpsc,
};

const CHUNK_SIZE: usize = 100;
const DISCOVERY_LENGTH: usize = 30;
const PLAYLIST_LENGTH: usize = 175;

type TrackList<'a> = Vec<PlayableId<'a>>;
type TrackSource<'a> = HashMap<Playlist, TrackList<'a>>;

trait TripleShuffle {
    fn triple_shuffle<R>(self, rng: &mut R) -> Self
    where
        R: rand::Rng + ?Sized;
}

impl<T> TripleShuffle for Vec<T> {
    fn triple_shuffle<R>(mut self, rng: &mut R) -> Self
    where
        R: rand::Rng + ?Sized
    {
        self.shuffle(rng);
        self.shuffle(rng);
        self.shuffle(rng);
        self
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
enum Playlist {
    CurrentLoop,
    FreshVibrations,
    IntoTheAbyss,
    FlowingAtmosphere,
    NerveRacking,
    DailyPlaylist,
}

impl Playlist {
    fn id(&self) -> AppResult<PlaylistId> {
        let id = match self {
            Playlist::CurrentLoop => "77JTZoDLsmXm1ODTdVc1oz",
            Playlist::FreshVibrations => "7tmG3W0fLJw9eDEaRCG8VY",
            Playlist::IntoTheAbyss => "0oc9wsvrxgwI17PCfbEo1l",
            Playlist::FlowingAtmosphere => "4Ty1f3XV2rOPrNOOBMPldQ",
            Playlist::NerveRacking => "1THuBLaWoC0E8PNo2MsFka",
            Playlist::DailyPlaylist => "42O1aSlfF0vlmLuBkPlcDO",
        };
        Ok(PlaylistId::from_id(id)?)
    }

    fn all() -> Vec<Playlist> {
        let mut variants = Vec::new();
        variants.push(Playlist::CurrentLoop);
        variants.push(Playlist::FreshVibrations);
        variants.push(Playlist::IntoTheAbyss);
        variants.push(Playlist::FlowingAtmosphere);
        variants.push(Playlist::NerveRacking);
        variants.push(Playlist::DailyPlaylist);
        variants
    }
}

struct PlaylistData<'a> {
    source: TrackSource<'a>,
    destination: TrackList<'a>,
}

impl<'a> PlaylistData<'a> {
    fn new() -> PlaylistData<'a> {
        PlaylistData {
            source: TrackSource::new(),
            destination: TrackList::new(),
        }
    }
}

pub async fn generate(spotify: Spotify) -> AppResult {
    let mut playlist_data = PlaylistData::new();
    fetch_playlist_data(&spotify, &mut playlist_data).await?;
    let track_selection = make_track_selection(playlist_data.source)?;
    clear_daily_playlist(&spotify, playlist_data.destination).await?;
    update_daily_playlist(&spotify, track_selection).await?;
    /*
    let (track_source, old_tracks) = fetch_playlist_data(&spotify).await?;
    let track_selection = make_track_selection(track_source)?;
    clear_daily_playlist(&spotify, old_tracks).await?;
    update_daily_playlist(&spotify, track_selection).await?;
    */
    Ok(())
}

async fn fetch_playlist_data<'a>(
    spotify: &Spotify,
    playlist_data: &mut PlaylistData<'a>
) -> AppResult {
    let playlists = Playlist::all();
    let mut streams = Vec::new();
    let (tx, rx) = mpsc::channel();
    for playlist in playlists.iter() {
        playlist_data.source.insert(playlist.clone(), TrackList::new());
        let playlist_id = playlist.id()?;
        let track_stream = spotify.playlist_items(playlist_id, None, None)
            .filter_map(|item| async {
                Some(item.ok()?.track?.id()?.clone_static())
            })
            .for_each(|track| {
                let tx = tx.clone();
                let playlist = playlist.clone();
                async move {
                    tx.send((playlist, track)).unwrap_or(());
                }
            })
            .into_future();
        streams.push(track_stream);
    }
    future::join_all(streams).await;
    drop(tx);
    while let Ok((id, track)) = rx.recv() {
        if let Some(track_list) = playlist_data.source.get_mut(&id) {
            track_list.push(track);
        }
    }
    playlist_data.destination = playlist_data.source
        .remove(&Playlist::DailyPlaylist)
        .ok_or(AppError::CouldNotGetPlaylist)?;
    Ok(())
}

fn make_track_selection(mut track_source: TrackSource) -> AppResult<TrackList> {
    let rng = &mut rand::rng();
    let mut selection = track_source
        .remove(&Playlist::CurrentLoop)
        .ok_or(AppError::CouldNotGetPlaylist)?;
    track_source
        .remove(&Playlist::FreshVibrations)
        .ok_or(AppError::CouldNotGetPlaylist)?
        .triple_shuffle(rng)
        .into_iter()
        .take(DISCOVERY_LENGTH)
        .for_each(|track| selection.push(track));
    track_source
        .into_iter()
        .map(|(_, track_list)| track_list)
        .flatten()
        .collect::<Vec<_>>()
        .triple_shuffle(rng)
        .into_iter()
        .take(PLAYLIST_LENGTH - selection.len())
        .for_each(|track| selection.push(track));
    Ok(selection.triple_shuffle(rng))
}

async fn clear_daily_playlist<'a>(
    spotify: &Spotify, old_tracks: TrackList<'a>
) -> AppResult {
    let playlist_id = Playlist::DailyPlaylist.id()?;
    for chunk in old_tracks.chunks(CHUNK_SIZE) {
        spotify.playlist_remove_all_occurrences_of_items(
            playlist_id.clone(), chunk.to_owned(), None
        ).await?;
    }
    Ok(())
}


async fn update_daily_playlist<'a>(
    spotify: &Spotify, selection: TrackList<'a>
) -> AppResult {
    let playlist_id = Playlist::DailyPlaylist.id()?;
    for chunk in selection.chunks(CHUNK_SIZE) {
        spotify.playlist_add_items(
            playlist_id.clone(), chunk.to_owned(), None
        ).await?;
    }
    Ok(())
}
