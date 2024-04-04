#[derive(Debug, thiserror::Error)]
pub enum WeatherError {
    #[error(transparent)]
    ApiError(#[from] reqwest::Error),
    #[error(transparent)]
    JoinTaskError(#[from] tokio::task::JoinError),
}