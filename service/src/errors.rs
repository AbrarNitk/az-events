#[derive(thiserror::Error, Debug)]
pub enum RouteError {
    #[error("JsonSerializeError: {0}")]
    JsonSerializeError(#[from] serde_json::Error),
    #[error("GetProfileError: {0}")]
    GetProfileError(#[from] http_service::controller::GetProfileError),
    #[error("BodyReadError: {0}")]
    BodyReadError(#[from] http_service::router::BodyError),
}
