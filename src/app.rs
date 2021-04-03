use crate::db::PgPool;
use crate::graphql::{create_schema, Context, Schema};

use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};

async fn index_handler() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn hello_handler(web::Path(name): web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello, {}", name.to_string()))
}

async fn graphql_handler(
    req: HttpRequest,
    payload: web::Payload,
    schema: web::Data<Schema>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    let context = Context::new(pool.get_ref().to_owned());

    juniper_actix::graphql_handler(&schema, &context, req, payload).await
}

async fn playground_handler() -> Result<HttpResponse, Error> {
    juniper_actix::playground_handler("/graphql", None).await
}

pub fn register(config: &mut web::ServiceConfig) {
    config
        .data(create_schema())
        .route("/", web::get().to(index_handler))
        .route("/hello", web::get().to(hello_handler))
        .route("/graphql", web::post().to(graphql_handler))
        .route("/graphiql", web::get().to(playground_handler));
}
