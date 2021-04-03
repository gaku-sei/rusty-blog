use crate::db::models::post::Post;
use juniper::GraphQLObject;

#[derive(GraphQLObject)]
#[graphql(description = "A simple blog post.")]
pub struct PostObject {
    #[graphql(description = "The blog id.")]
    pub id: i32,
    #[graphql(description = "The blog title.")]
    pub title: String,
    #[graphql(description = "The blog body.")]
    pub body: String,
    #[graphql(description = "The blog publication status.")]
    pub published: bool,
}

impl From<Post> for PostObject {
    fn from(post: Post) -> Self {
        PostObject {
            id: post.id,
            title: post.title.clone(),
            body: post.body.clone(),
            published: post.published,
        }
    }
}
