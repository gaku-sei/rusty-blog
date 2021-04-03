use super::object::PostObject;
use super::Context;
use crate::db::models::post::{NewPost, Post};
use juniper::{graphql_object, FieldResult};

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    #[graphql(
        description = "Creates a blog post.",
        arguments(
            title(description = "The blog title."),
            body(description = "The blog body.")
        )
    )]
    fn create_post(context: &Context, title: String, body: String) -> FieldResult<PostObject> {
        let conn = context.pool.get()?;

        let post = Post::create(&conn, NewPost::new(title, body))?;

        Ok(post.into())
    }
}
