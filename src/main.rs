pub mod database;
pub mod graphql;
pub mod handler;
pub mod models;

//Libray Import
use actix_cors::Cors;
use actix_web::{
    get, guard, middleware::Logger, post, route, web, App, HttpResponse, HttpServer, Responder,
};
extern crate colored;
use actix_web_lab::respond::Html;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use colored::*;

//Local imports
use crate::graphql::MainSchema;
// use crate::models::user::UserGQL;
use database::db_pool;
use graphql::{RootMutation, RootQuery};
use handler::{gql_playgound, index};
use mongodb::Client;
// use std::sync::*;

//======================**GraphQL endpoint**=======================
#[route("/graphql", method = "POST")]
async fn graphqls(
    schema: web::Data<MainSchema>,
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

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
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
        "/graphiql".green().on_bright_cyan()
    );

    let schema = Schema::build(RootQuery, RootMutation, EmptySubscription)
        // .data(db_pool())
        .finish();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .app_data(web::Data::new(pool.clone()))
            // .app_data(web::Data::new(pool.clone()))
            // .app_data(client.clone())
            // .service(graphqls)
            // .service(graphql_playground)
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["POST", "GET"]),
            )
            .wrap(Logger::default())
            // .service(hello)
            // .service(echo)
            .service(web::resource("/api").guard(guard::Post()).to(index))
            .service(web::resource("/api").guard(guard::Get()).to(gql_playgound))
            .route("/hey", web::get().to(manual_hello))
    })
    .workers(2)
    .bind(&address)?
    .run()
    .await
}
