pub mod database;
mod graphql;
mod handler;
pub mod models;

//Libray Import
use actix_cors::Cors;
use actix_web::{get, post, route, web, App, HttpResponse, HttpServer, Responder};
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
use database::db_pool;
use graphql::{RootMutation, RootQuery};
// use handler::{gql_playgound, index};

//======================**GraphQL endpoint**=======================
#[route("/graphql", method = "GET", method = "POST")]
async fn graphqls(schema: web::Data<MainSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
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
    // std::env::set_var("RUST_LOG", "actix_web=debug");
    // let mut client_options = ClientOptions::parse("mongouri").await.unwrap();
    // client_options.app_name = Some("salaManagement".to_string());
    // let client = web::Data::new(Mutex::new(Client::with_options(client_options).unwrap()));

    dotenv::from_filename(".env").ok();
    let ip = dotenv::var("IP").unwrap();
    let port = dotenv::var("PORT").unwrap();
    let address = format!("{}:{}", ip, port);

    let _pool = db_pool().await.unwrap();

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
            .app_data(schema.clone())
            .app_data(_pool.clone())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["POST", "GET"])
                    .allow_any_header()
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(hello)
            .service(echo)
            .service(graphqls)
            .service(graphql_playground)
            // .service(web::resource("/api").guard(guard::Post()).to(index))
            // .service(web::resource("/api").guard(guard::Get()).to(gql_playgound))
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(&address)?
    .run()
    .await
}
