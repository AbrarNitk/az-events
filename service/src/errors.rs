#[derive(thiserror::Error, Debug)]
pub enum RouteError {
    #[error("JsonSerializeError: {0}")]
    JsonSerializeError(#[from] serde_json::Error),
    #[error("BodyReadError: {0}")]
    BodyReadError(#[from] service::router::BodyError),
}
