use diesel::prelude::*;
use diesel::RunQueryDsl;

pub fn create(
    pool: &crate::pg::DbPool,
    ekind: &str,
    edata: &serde_json::Value,
) -> Result<(), crate::DBError> {
    use crate::schema::events;

    let mut conn = pool
        .get()
        .map_err(|x| crate::DBError::PooledConnection(x.to_string()))?;

    diesel::insert_into(events::dsl::events)
        .values((
            events::dsl::ekind.eq(ekind),
            events::dsl::edata.eq(edata),
            events::dsl::created_on.eq(chrono::Utc::now()),
        ))
        .execute(&mut conn)?;

    Ok(())
}
