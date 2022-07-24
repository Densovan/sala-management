
use async_graphql::{EmptySubscription, Schema};
use mongodb::Client;
use serde_derive::{Deserialize, Serialize};

//Local import
use crate::graphql::{RootMutation, RootQuery};

pub type BuildSchema = Schema<RootQuery, RootMutation, EmptySubscription>;

pub struct Token(pub String);

pub struct AppContext {
    pub db_pool: Client,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Claims {
    pub exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub iat: usize, // Optional. Issued at (as UTC timestamp)
    pub iss: String, // Optional. Issuer
    pub sub: String, // Optional. Subject (whom token refers to)
}
