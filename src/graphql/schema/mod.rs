mod mutation;
mod object;
mod query;

use super::Context;
use juniper::EmptySubscription;
use mutation::*;
use query::*;

pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}
