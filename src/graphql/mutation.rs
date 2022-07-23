use async_graphql::*;
use bcrypt::{hash, verify};
use dotenv::dotenv;

use mongodb::bson::{doc};
use std::env;

//MODELS
use super::AppContext;
use super::{Claims, RootQuery};

use crate::models::{user::UserGQL};
use crate::utils::{ message::Message };
use jsonwebtoken::{encode, EncodingKey, Header};
pub struct RootMutation;

#[Object]
impl RootMutation {
    async fn signup(
        &self,
        ctx: &Context<'_>,
        // #[validate(email)] email: String,
        email: String,
        password: String,
        fullname: String,
        // gender: String,
        // phone: String,
    ) -> FieldResult<UserGQL> {
        let db = ctx.data_unchecked::<AppContext>().db_pool.clone();
        let collection = db.database("rusttest").collection("users");

        //----------------> chekc email exist <-----------------------
        let existed_email = RootQuery.user_by_email(ctx, email.clone()).await;

        match existed_email {
            Ok(_) => Err(FieldError::from("Email already in exist")),
            Err(_) => {
                let new_user = doc! {
                    "email":email.to_string(),
                    "password":hash(password.to_string(), 6).unwrap(),
                    "fullname":fullname.to_string(),
                    "gender":"",
                    "phone":"",
                };
                #[allow(unused_assignments)]
                let mut _new_user_id: String = String::from("");
                let result = collection.insert_one(new_user, None).await;
                println!("{:#?}", result);
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
                    phone: String::from(""),
                    gender: String::from(""),
                    password,
                })
            }
        }
    }
    // sign in user

    pub async fn sign_in(
        &self,
        ctx: &Context<'_>,
        email: String,
        password: String,
    ) -> FieldResult<String> {
        dotenv().ok();
        // let my_iat = Utc::now().timestamp();
        // let my_exp = Utc::now()
        //     .checked_add_signed(Duration::seconds(5))
        //     .expect("invalid timestamp")
        //     .timestamp();
        match RootQuery.user_by_email(ctx, email).await {
            Ok(data) => match verify(password, &data.password).unwrap() {
                true => {
                    #[allow(non_snake_case)]
                    let jwtsecret = env::var("JWTSECRET").unwrap();

                    let option = Claims::default();
                    let token = encode(
                        &Header::default(),
                        &option,
                        &EncodingKey::from_secret(jwtsecret.as_ref()),
                    )
                    .unwrap();
                    println!("{}", &token);
                    Ok(token.to_string())
                }
                false => Err(FieldError::from("Invalid password")),
            },
            Err(e) => Err(FieldError::from(e)),
        }
    }

    pub async fn create_session(
        &self,
        ctx: &Context<'_>,
        room_id: String,
        lesson_id: String,
    ) -> FieldResult<Message> {

        dotenv().ok();

        let db = ctx.data_unchecked::<AppContext>().db_pool.clone();
        let collection = db.database("rusttest").collection("sessions");

        let new_session = doc! {
            "room_id": room_id.to_string(),
            "lesson_id": lesson_id.to_string(),
        };

        // #[allow(unused_assignments)]
        let mut _new_session: String = String::from("");
        let result = collection.insert_one(new_session, None).await;
        // let result = collection.insert_one(new_section, None).await.expect("Error Insert!");
        println!("{:#?}", _new_session);

        match result {
            Ok(data) => {
                let results = data.inserted_id.as_object_id();
                _new_session = results.unwrap().to_string();
            }
            Err(err) => {
                println!("{:?}", err)
            }
        }
        Ok(Message{message: String::from("Create session successfully!")})
    }
}
