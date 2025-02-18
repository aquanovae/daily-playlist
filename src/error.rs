use thiserror::Error;



pub type AppResult<T = ()> = Result<T, AppError>;


#[derive(Debug, Error)]
pub enum AppError {

    #[error("{0}")]
    BaseDirectoriesError(#[from] xdg::BaseDirectoriesError),

    #[error("")]
    CouldNotGetPlaylist,

    #[error("")]
    CouldNotGetTrackList,

    #[error("{0}")]
    ClientError(#[from] rspotify::ClientError),

    #[error("{0}")]
    IdError(#[from] rspotify::model::idtypes::IdError),

    #[error("")]
    InvalidCallbackAddress,

    #[error("{0}")]
    IoEror(#[from] std::io::Error),

    #[error("{0}")]
    ModelError(#[from] rspotify::model::error::ModelError),
}
