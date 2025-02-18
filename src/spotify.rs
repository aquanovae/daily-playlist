use crate::AppResult;

use rspotify::{
    AuthCodePkceSpotify, Config, Credentials, OAuth, Token,
    clients::OAuthClient,
};



pub type Spotify = AuthCodePkceSpotify;



const CLIENT_ID: &str = "207ee9e318444985827ba5c3c9cb3d92";
const CALLBACK_ADDRESS: &str = "http://127.0.0.1:8080";



pub async fn authenticate_user(config: Config) -> AppResult {

    let mut spotify = connect_to_api(None, config);
    let authorize_url = spotify.get_authorize_url(None)?;
    let code = spotify.get_code_from_user(&authorize_url)?;
    spotify.request_token(&code).await?;
    Ok(())
}


pub fn connect_to_api(token: Option<Token>, config: Config) -> AuthCodePkceSpotify {

    let credentials = Credentials::new_pkce(CLIENT_ID);
    let oauth = OAuth {
        redirect_uri: CALLBACK_ADDRESS.to_owned(),
        scopes: rspotify::scopes!(
            "playlist-read-private",
            "playlist-modify-private"
        ),
        ..Default::default()
    };

    match token {
        Some(token) => AuthCodePkceSpotify::from_token_with_config(token, credentials, oauth, config),
        None => AuthCodePkceSpotify::with_config(credentials, oauth, config),
    }
}
