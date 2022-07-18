// use actix_web::HttpResponse;
//Libray Imports
extern crate async_graphql;
use async_graphql::ID;
use async_graphql::*;

// use async_graphql::validators::{max_length};
// use async_graphql::{
//     Context, FieldError, FieldResult, Object, ID,
// };
// use bcrypt::{hash, verify};
// use bson;
// use futures::future::ok;
use mongodb::bson::doc;
//use std::sync::Arc;
//MODELS
// use super::{Claim,RootQuery}

use crate::models::user::UserGQL;

// use jsonwebtoken::{encode, EncodingKey, Header};

use super::AppContext;

pub struct RootMutation;

#[Object]
impl RootMutation {
    async fn signup(
        &self,
        ctx: &Context<'_>,
        email: String,
        password: String,
        fullname: String,
        gender: String,
        phone: String,
    ) -> FieldResult<UserGQL> {
        let db = ctx.data_unchecked::<AppContext>().db_pool.clone();
        let collection = db.database("rusttest").collection("users");
        let new_user = doc! {
            "email":email.to_string(),
            "password":password.to_string(),
            "fullname":fullname.to_string(),
            "gender":gender.to_string(),
            "phone":phone.to_string(),
        };
        let mut _new_user_id: String = String::from("");
        let result = collection.insert_one(new_user, None).await;
        match result {
            Ok(data) => {
                let results = data.inserted_id.as_object_id();
                _new_user_id = results.unwrap().to_string();
            }
            Err(err) => {
                println!("{:?}", err)
            }
        }
        Ok(UserGQL {
            id: ID::from("001"),
            fullname,
            email,
            password,
            phone,
            gender,
        })
    }
}
