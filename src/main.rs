mod error;
mod playlist;
mod spotify;

pub use error::{ AppError, AppResult };
pub use spotify::Spotify;

use clap::{ Parser, Subcommand };

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
    match Cli::parse().command {
        Commands::Login => {
            let _ = spotify::connect_to_api().await?;
        },
        Commands::Generate => {
            let spotify = spotify::connect_to_api().await?;
            playlist::generate(spotify).await?;
        },
    };
    Ok(())
}
