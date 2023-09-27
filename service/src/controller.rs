#[derive(serde::Deserialize, Debug)]
pub struct RequestEvent {
    ekind: String,
    edata: Edata,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Edata {
    path: String,
    method: String,
}

#[derive(thiserror::Error, Debug)]
pub enum HandleEventError {
    #[error("SerdeJsonError: {}", _0)]
    SerdeJson(#[from] serde_json::Error),
    #[error("DbError: {}", _0)]
    Db(#[from] db::DBError),
}

pub async fn handle_event(
    config: &crate::Config,
    event: RequestEvent,
) -> Result<(), HandleEventError> {
    db::events::create(
        &config.db_pool,
        event.ekind.as_str(),
        &serde_json::to_value(event.edata)?,
    )?;
    Ok(())
}
