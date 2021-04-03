use crate::db::schema::posts;
use diesel::prelude::*;

#[derive(Clone, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost {
    title: String,
    body: String,
    published: bool,
}

impl NewPost {
    pub fn new(title: String, body: String) -> Self {
        NewPost {
            title,
            body,
            published: false,
        }
    }
}

impl Post {
    pub fn get_all(conn: &PgConnection, published: bool) -> QueryResult<Vec<Post>> {
        posts::dsl::posts
            .filter(posts::published.eq(published))
            .limit(5)
            .load::<Post>(conn)
    }

    pub fn create(conn: &PgConnection, new_post: NewPost) -> QueryResult<Post> {
        diesel::insert_into(posts::table)
            .values(&new_post)
            .get_result(conn)
    }
}
