use crate::AppResult;

use rspotify::{
    AuthCodePkceSpotify, Config, Credentials, OAuth,
    clients::OAuthClient,
};

use std::path::PathBuf;



pub type Spotify = AuthCodePkceSpotify;



const CLIENT_ID: &str = "207ee9e318444985827ba5c3c9cb3d92";
const CALLBACK_ADDRESS: &str = "http://dummy.dummy";



pub async fn connect_to_api() -> AppResult<Spotify> {

    let mut cache_path = PathBuf::from("/var/cache/daily-playlist");
    match std::fs::exists(&cache_path) {
        Err(_) | Ok(false) => std::fs::create_dir_all(&cache_path)?,
        _ => ()
    };
    cache_path.push("token.json");
    let credentials = Credentials::new_pkce(CLIENT_ID);
    let oauth = OAuth {
        redirect_uri: CALLBACK_ADDRESS.to_owned(),
        scopes: rspotify::scopes!(
            "playlist-read-private",
            "playlist-modify-private"
        ),
        ..Default::default()
    };
    let config = Config {
        cache_path,
        token_cached: true,
        token_refreshing: true,
        ..Default::default()
    };
    let mut spotify = Spotify::with_config(credentials, oauth, config);
    let authorize_url = spotify.get_authorize_url(None)?;
    spotify.prompt_for_token(&authorize_url).await?;

    Ok(spotify)
}
