mod error;
mod playlist;
mod spotify;

pub use error::{ AppError, AppResult };
pub use spotify::Spotify;


use clap::{ Parser, Subcommand };
use rspotify::{ Config, Token };



#[derive(Parser)]
struct Cli {

    #[command(subcommand)]
    command: Commands,
}


#[derive(Subcommand)]
enum Commands {

    /// Login spotify account
    Login,

    /// Generate new playlist
    Generate,
}



#[tokio::main]
async fn main() -> AppResult {

    let mut cache_path = xdg::BaseDirectories::new()?
        .create_cache_directory("daily-playlist")?;
    cache_path.push("token.json");

    let config = Config {
        cache_path: cache_path.clone(),
        token_cached: true,
        token_refreshing: true,
        ..Default::default()
    };


    match Cli::parse().command {
        Commands::Login => {
            spotify::authenticate_user(config).await?
        },
        Commands::Generate => {
            let token = Token::from_cache(cache_path)?;
            let spotify = spotify::connect_to_api(Some(token), config);
            playlist::generate(spotify).await?;
        },
    };

    Ok(())
}
