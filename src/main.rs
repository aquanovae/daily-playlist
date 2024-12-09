// =============================================================================
//
// Generate a spotify playlist with random tracks from multiple playlists
//
// Functions:
//  main
//  update_daily_playlist
//  track_selection
//  track_source
//  track_list
//  playlist_id
//  spotify_api
//
// Enums:
//  Playlist
// =============================================================================

use std::collections::HashMap;

use rand::seq::SliceRandom;

use rspotify::{ AuthCodePkceSpotify, Config, Credentials, OAuth, scopes };
use rspotify::clients::{ BaseClient, OAuthClient };
use rspotify::model::{ PlayableId, PlaylistId };

// =============================================================================

fn main() {

    let spotify_api = spotify_api();

    let track_selection = track_selection(&spotify_api);

    update_daily_playlist(&spotify_api, track_selection);
}

// =============================================================================
// Clear all tracks in output playlist and add newly generated selection
//
fn update_daily_playlist(
    spotify_api: &AuthCodePkceSpotify,
    tracks: Vec<PlayableId>

) {

    let playlist_id = playlist_id(Playlist::DailyPlaylist);

    let tracks_to_clear = track_list(&spotify_api, Playlist::DailyPlaylist);

    let chunk_size = 75;


    for tracks_chunk in tracks_to_clear.chunks(chunk_size) {

        let tracks = tracks_chunk
            .iter()
            .map( |playable_id|
                  playable_id.clone_static()
            )
            .collect::<Vec<_>>();

        spotify_api
            .playlist_remove_all_occurrences_of_items(
                playlist_id.clone(),
                tracks,
                None
            )
            .expect("Could not clear playlist");
    }


    for (i, tracks_chunk) in tracks.chunks(chunk_size).enumerate() {

        let tracks = tracks_chunk
            .iter()
            .map( |playable_id|
                  playable_id.clone_static()
            )
            .collect::<Vec<_>>();

        let start_index = (i * chunk_size) as u32;

        spotify_api
            .playlist_add_items(
                playlist_id.clone(),
                tracks,
                Some(start_index)
            )
            .expect("Could not add selection to playlist");
    }

    println!("Daily playlist updated");
}

// =============================================================================
// Select random tracks in multiple playlists and shuffle them
//
fn track_selection(
    spotify_api: &AuthCodePkceSpotify

) -> Vec<PlayableId> {

    let mut rng = rand::thread_rng();

    let mut track_selection = Vec::new();

    for (playlist, track_list) in track_source(&spotify_api) {

        let track_count = match playlist {
            Playlist::CurrentLoop       => 40,
            Playlist::FreshVibrations   => 20,
            Playlist::IntoTheAbyss      => 30,
            Playlist::FlowingAtmosphere => 30,
            Playlist::NerveRacking      => 30,
            _ => 0
        };

        let mut selection = track_list
            .choose_multiple(&mut rng, track_count)
            .map( |track_id|
                  track_id.clone_static()
            )
            .collect::<Vec<_>>();

        track_selection.append(&mut selection);
    }

    track_selection.shuffle(&mut rng);

    println!("Track selection done");
    track_selection
}

// =============================================================================
// List all possible tracks for selection sorted by playlist
//
fn track_source(
    spotify_api: &AuthCodePkceSpotify

) -> HashMap<Playlist, Vec<PlayableId>> {

    let sources = vec![
        Playlist::CurrentLoop,
        Playlist::FreshVibrations,
        Playlist::IntoTheAbyss,
        Playlist::FlowingAtmosphere,
        Playlist::NerveRacking
    ];

    let mut track_source = HashMap::new();

    for playlist in sources {

        let track_list = track_list(&spotify_api, playlist);

        track_source.insert(playlist, track_list);
    }

    track_source
}

// =============================================================================
// Fetch track list of a given playlist
//
fn track_list(
    spotify_api: &AuthCodePkceSpotify,
    playlist: Playlist

) -> Vec<PlayableId> {

    let playlist_id = playlist_id(playlist);

    let track_list = spotify_api
        .playlist_items(playlist_id, None, None)
        .map( |playlist_item| {

            let playlist_item = playlist_item
                .expect("Could not get playlist item");

            let playable = playlist_item.track
                .expect("Could not get track info");

            playable.id()
                .expect("Could not get track ID")
                .clone_static()
        })
        .collect::<Vec<_>>();

    println!("Retrieved track list of {:?}", playlist);
    track_list
}

// =============================================================================
// Playlists that can be accessed
//
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
enum Playlist {
    CurrentLoop,
    FreshVibrations,
    IntoTheAbyss,
    FlowingAtmosphere,
    NerveRacking,
    DailyPlaylist
}

// =============================================================================
// Return spotify playlist ID for enum variant
//
fn playlist_id(
    playlist: Playlist

) -> PlaylistId<'static> {

    let playlist_id = match playlist {
        Playlist::CurrentLoop       => "77JTZoDLsmXm1ODTdVc1oz",
        Playlist::FreshVibrations   => "7tmG3W0fLJw9eDEaRCG8VY",
        Playlist::IntoTheAbyss      => "0oc9wsvrxgwI17PCfbEo1l",
        Playlist::FlowingAtmosphere => "4Ty1f3XV2rOPrNOOBMPldQ",
        Playlist::NerveRacking      => "1THuBLaWoC0E8PNo2MsFka",
        Playlist::DailyPlaylist     => "42O1aSlfF0vlmLuBkPlcDO"
    };

    let playlist_id = PlaylistId::from_id(playlist_id)
        .expect("Invalid playlist ID");

    playlist_id
}

// =============================================================================
// Connect to spotify API
//
fn spotify_api()

-> AuthCodePkceSpotify {

    let credentials = Credentials::new_pkce("207ee9e318444985827ba5c3c9cb3d92");

    let oauth = OAuth {
        redirect_uri: String::from("https://localhost/callback"),
        scopes: scopes!(
            "playlist-read-private",
            "playlist-modify-private"
        ),
        ..Default::default()
    };

    let mut spotify_api = AuthCodePkceSpotify::with_config(
        credentials,
        oauth,
        Config {
            token_cached: true,
            token_refreshing: true,
            ..Default::default()
        }
    );

    let authorize_url = spotify_api.get_authorize_url(None)
        .expect("Could not get url from Spotify");

    spotify_api.prompt_for_token(&authorize_url)
        .expect("User authentification failed");

    println!("Successful spotify authentication");
    spotify_api
}
