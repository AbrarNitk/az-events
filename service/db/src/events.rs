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

// event count by path
// event count by id
// let's keep count by id fo now
// later we have to build a follower that will update the count for pages and all

pub fn count(pool: &crate::pg::DbPool, ekind: &str) -> Result<i64, crate::DBError> {
    // sql_function!(fn jsonb_path_match(jsonb: diesel::sql_types::Jsonb, path: diesel::sql_types::Text) -> Text);

    use crate::schema::events;
    let mut conn = pool
        .get()
        .map_err(|x| crate::DBError::PooledConnection(x.to_string()))?;
    let r = events::dsl::events
        .select(diesel::dsl::count(events::dsl::id))
        .filter(events::dsl::ekind.eq(ekind))
        // .group_by(jsonb_path_match(events::edata, "path"))
        .first(&mut conn)?;
    println!("{:?}", r);
    Ok(r)
}
