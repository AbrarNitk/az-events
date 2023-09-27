extern crate self as http_service;

pub mod controller;
pub mod errors;
pub mod router;
#[macro_use]
pub mod macros;
pub mod utils;

pub struct Config {
    pub db_pool: db::pg::DbPool
}