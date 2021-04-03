use crate::db::PgPool;

pub struct Context {
    pub pool: PgPool,
}

impl Context {
    pub fn new(pool: PgPool) -> Self {
        Context { pool }
    }
}

impl juniper::Context for Context {}
