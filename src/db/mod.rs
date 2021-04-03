use super::config;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub mod models;
pub mod schema;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn connect() -> PgPool {
    let manager = ConnectionManager::new(config::DATABASE::URL());

    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
