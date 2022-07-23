mod graphql;
mod models;
mod utils;
mod config;

use actix_cors::Cors;
use actix_web::{
    get, guard, middleware::Logger, route, web, App, HttpResponse, HttpServer, Responder,
};
extern crate colored;
use actix_web_lab::respond::Html;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use colored::*;

use crate::graphql::{ BuildSchema, RootMutation, RootQuery };
use crate::config::{ database::db_pool };
use crate::utils::{handler::{gql_playground, index}};
use mongodb::Client;

//======================GraphQL endpoint=======================
#[route("/graphql", method = "POST")]
async fn graphqls(
    schema: web::Data<BuildSchema>,
    db_pool: web::Data<Client>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = req.into_inner();
    request = request.data(db_pool);

    schema.execute(request).await.into()
}
/// GraphiQL playground UI
#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
    ))
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    dotenv::from_filename(".env").ok();
    let ip = dotenv::var("IP").unwrap();
    let port = dotenv::var("PORT").unwrap();
    let address = format!("{}:{}", ip, port);

    let pool = db_pool().await.unwrap();

    println!(
        "{}{}",
        "Server is running at: http://".red().on_blue(),
        &address.red().on_blue()
    );
    println!(
        "{}{}{}",
        "GraphQL is running at: http://".green().on_bright_cyan(),
        &address.green().on_bright_cyan(),
        "/api".green().on_bright_cyan()
    );

    let schema = Schema::build(RootQuery, RootMutation, EmptySubscription).finish();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .app_data(web::Data::new(pool.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["POST", "GET"]),
            )
            .wrap(Logger::default())
            .service(web::resource("/api").guard(guard::Post()).to(index))
            .service(web::resource("/api").guard(guard::Get()).to(gql_playground))
            .route("/", web::get().to(hello))
    })
    .workers(2)
    .bind(&address)?
    .run()
    .await
}