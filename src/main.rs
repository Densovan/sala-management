pub mod models;
pub mod database;
use database::db_pool;
//Libray Import 
use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


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
    // let mut client_options = ClientOptions::parse("mongodb+srv://den:sarimsovanden9999086280018@blogs.eqoih.mongodb.net/rusttest?retryWrites=true&w=majority").await.unwrap();
    // client_options.app_name = Some("salaManagement".to_string());
    // let client = web::Data::new(Mutex::new(Client::with_options(client_options).unwrap()));


   dotenv::from_filename(".env").ok();
   let ip = dotenv::var("IP").unwrap();
   let port = dotenv::var("PORT").unwrap();
   let address = format!("{}:{}", ip, port);

   let _pool = db_pool().await.unwrap();

   println!("Server is running at: http://{}", &address);
    println!("GraphQL is running at: http://{}/api", &address);


    HttpServer::new(move || {
        App::new()
            .app_data(_pool.clone())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["POST","GET"])
                    .allow_any_header()
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(&address)?
    .run()
    .await
}