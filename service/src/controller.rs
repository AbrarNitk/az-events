#[derive(serde::Deserialize, Debug)]
pub struct RequestEvent {
    ekind: String,
    edata: Edata,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Edata {
    path: String,
    method: String,
    user_agent: Option<String>,
    referer: Option<String>,
    // x-real-ip
    ip: Option<String>,
    // x-forwarded-for
    forwarder_for: Option<String>,
    // sec-ch-ua-platform
    platform: Option<String>,
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

#[derive(serde::Serialize, Debug)]
pub struct GetEventResponse {
    count: i64,
}

#[derive(thiserror::Error, Debug)]
pub enum GetEventError {
    #[error("DBError: {}", _0)]
    DBError(#[from] db::DBError),
}

pub async fn get_events(config: &crate::Config) -> Result<GetEventResponse, GetEventError> {
    Ok(GetEventResponse {
        count: db::events::count(&config.db_pool, "page_visit")?,
    })
}
