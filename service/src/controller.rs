#[derive(Default, Debug, serde::Serialize)]
pub struct GetProfileResponse {}

#[derive(thiserror::Error, Debug)]
pub enum GetProfileError {}

pub fn get_user_profile() -> Result<GetProfileResponse, GetProfileError> {
    Ok(GetProfileResponse::default())
}


#[derive(serde::Deserialize, Debug)]
pub struct RequestEvent {
    ekind: String,
    edata: Edata,
}

#[derive(serde::Deserialize, Debug)]
pub struct Edata {
    route: String,
}


#[derive(thiserror::Error, Debug)]
pub enum HandleEventError {}

pub async fn handle_event(event: RequestEvent) -> Result<(), HandleEventError> {
    Ok(())
}