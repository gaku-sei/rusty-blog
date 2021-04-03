use super::object::PostObject;
use super::Context;
use crate::db::models::post::Post;
use juniper::{graphql_object, FieldResult};

#[derive(Debug)]
pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    #[graphql(
        description = "Get all the posts.",
        arguments(published(
            default = true,
            description = "If true returns only published posts, defaults to true."
        ))
    )]
    fn posts(context: &Context, published: bool) -> FieldResult<Vec<PostObject>> {
        let conn = context.pool.get()?;

        let posts = Post::get_all(&conn, published)?
            .iter()
            .map(|post| PostObject::from(post.clone()))
            .collect();

        Ok(posts)
    }
}
