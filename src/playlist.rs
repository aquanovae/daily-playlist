use crate::{ AppError, AppResult, Spotify };

use futures::stream::StreamExt;

use rand::seq::{ IteratorRandom, SliceRandom };

use rspotify::{
    clients::{ BaseClient, OAuthClient },
    model::PlaylistId,
};

use std::{
    collections::HashMap,
    future::IntoFuture,
};



type PlayableId = rspotify::model::PlayableId<'static>;
type TrackList = Vec<PlayableId>;
type TrackSource = HashMap<String, TrackList>;



const CHUNK_SIZE: usize = 100;
const DESTINATION_PLAYLIST_ID: &str = "42O1aSlfF0vlmLuBkPlcDO";
const PLAYLISTS: [PlaylistData; 6] = [
    PlaylistData{ id: "77JTZoDLsmXm1ODTdVc1oz", choose_tracks: 80 }, // Current Loop
    PlaylistData{ id: "7tmG3W0fLJw9eDEaRCG8VY", choose_tracks: 35 }, // Fresh Vibrations
    PlaylistData{ id: "0oc9wsvrxgwI17PCfbEo1l", choose_tracks: 20 }, // Into The Abyss
    PlaylistData{ id: "4Ty1f3XV2rOPrNOOBMPldQ", choose_tracks: 20 }, // Flowing Atmosphere
    PlaylistData{ id: "1THuBLaWoC0E8PNo2MsFka", choose_tracks: 20 }, // Nerve Racking
    PlaylistData{ id: "42O1aSlfF0vlmLuBkPlcDO", choose_tracks: 0  }, // Daily Playlist
];

struct PlaylistData {
    id: &'static str,
    choose_tracks: usize,
}



pub async fn generate(spotify: Spotify) -> AppResult {

    let (track_source, old_tracks) = fetch_playlist_data(&spotify).await?;
    let track_selection = make_track_selection(track_source)?;

    clear_daily_playlist(&spotify, old_tracks).await?;
    update_daily_playlist(&spotify, track_selection).await?;

    Ok(())
}


async fn fetch_playlist_data(
    spotify: &Spotify
) -> AppResult<(TrackSource, TrackList)> {

    let (tx, rx) = std::sync::mpsc::channel();
    let mut track_source = HashMap::new();
    let mut streams = Vec::new();

    for PlaylistData{ id, .. } in &PLAYLISTS {
        track_source.insert(id.to_string(), Vec::new());
        let playlist_id = PlaylistId::from_id(*id)?;
        let track_stream = spotify.playlist_items(playlist_id, None, None)
            .filter_map(|item| async {
                Some(item.ok()?.track?.id()?.into_static())
            })
            .for_each(|track| {
                let tx = tx.clone();
                let id = *id;
                async move {
                    tx.send((id, track)).unwrap();
                }
            })
            .into_future();

        streams.push(track_stream);
    }

    futures::future::join_all(streams).await;
    drop(tx);

    while let Ok((id, track)) = rx.recv() {
        track_source.get_mut(id).unwrap().push(track);
    }

    let old_tracks = track_source.remove(DESTINATION_PLAYLIST_ID).unwrap();

    Ok((track_source, old_tracks))
}


fn make_track_selection(mut track_source: TrackSource) -> AppResult<TrackList> {

    let rng = &mut rand::rng();
    let mut selection = Vec::new();

    for PlaylistData{ id, choose_tracks } in PLAYLISTS {
        if choose_tracks == 0 { continue }

        let track_list = track_source.remove(id)
            .ok_or(AppError::CouldNotGetPlaylist)?;
        let mut random_selection = track_list.into_iter()
            .choose_multiple(rng, choose_tracks);
        selection.append(&mut random_selection);
    }

    selection.shuffle(rng);

    Ok(selection)
}


async fn clear_daily_playlist(
    spotify: &Spotify, old_tracks: TrackList
) -> AppResult {

    let playlist_id = PlaylistId::from_id(DESTINATION_PLAYLIST_ID)?;
    println!("{}", old_tracks.len());

    for chunk in old_tracks.chunks(CHUNK_SIZE) {
        spotify.playlist_remove_all_occurrences_of_items(
            playlist_id.clone(), chunk.to_owned(), None
        ).await?;
    }

    Ok(())
}


async fn update_daily_playlist(
    spotify: &Spotify, selection: TrackList
) -> AppResult {

    let playlist_id = PlaylistId::from_id(DESTINATION_PLAYLIST_ID)?;

    for chunk in selection.chunks(CHUNK_SIZE) {
        spotify.playlist_add_items(
            playlist_id.clone(), chunk.to_owned(), None
        ).await?;
    }

    Ok(())
}
