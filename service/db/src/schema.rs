// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Int8,
        #[max_length = 127]
        ekind -> Text,
        edata -> Jsonb,
        created_on -> Timestamptz,
    }
}
