extern crate colored;
use colored::*;

use mongodb::{error::Error, options::ClientOptions, Client};

pub async fn db_pool() -> Result<Client, Error> {
    dotenv::from_filename(".env").ok();

    let db = dotenv::var("MONGOURI").unwrap();

    let db_address = format!("{}", db = db,);

    let mut client_options = ClientOptions::parse(&db_address).await?;
    client_options.retry_writes = Some(false);
    let client = Client::with_options(client_options);

    match client {
        Ok(c) => {
            println!("{}", "Connected to database".green().on_bright_green());
            Ok(c)
        }
        Err(e) => Err(e),
    }
}
