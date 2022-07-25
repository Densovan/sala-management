use async_graphql::*;
use bcrypt::{hash, verify};
use bson::{doc, Bson};
use dotenv::dotenv;
use std::env;

//MODELS
use super::AppContext;
use super::{Claims, RootQuery};
use crate::models::classroom::{ClassroomGQL, ClassroomModel};
use crate::models::user::UserGQL;
// use crate::utils::message::Message;
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
        // let my_iat = Utc::now().date();
        // let my_exp = Utc::now()
        //     .checked_add_signed(Duration::seconds(5))
        //     .expect("invalid date")
        //     .date();
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

    //========================>>ClassRoom Section<<============================
    pub async fn create_class_room(
        &self,
        ctx: &Context<'_>,
        name: String,
        school_id: String,
    ) -> FieldResult<ClassroomGQL> {
        let db = ctx.data_unchecked::<AppContext>().db_pool.clone();
        let collection = db.database("rusttest").collection("classrooms");

        let new_class_room = doc! {
            "name":name.to_string(),
            "school_id":school_id.to_string(),
            "date":bson::DateTime::now()
        };
        #[allow(unused_assignments)]
        let mut _class_id: String = String::from("");
        let result = collection.insert_one(new_class_room, None).await;
        println!("{:#?}", result);
        match result {
            Ok(data) => {
                let results = data.inserted_id.as_object_id();
                _class_id = results.unwrap().to_string();
            }
            Err(err) => {
                println!("{:?}", err)
            }
        }
        Ok(ClassroomGQL {
            id: ID::from("001"),
            name,
            school_id,
            date: String::from(""),
            message: String::from("successfully"),
        })
    }

    pub async fn update_classroom(
        &self,
        ctx: &Context<'_>,
        id: String,
        name: String,
    ) -> FieldResult<ClassroomGQL> {
        let db = ctx.data_unchecked::<AppContext>().db_pool.clone();
        let collection = db.database("rusttest").collection("classrooms");

        let converted_id = match bson::oid::ObjectId::parse_str(&id) {
            Ok(data) => data,
            Err(_) => return Err(FieldError::from("Not a valid id")),
        };

        let cursor = collection
            .find_one_and_update(
                doc! {"_id":converted_id},
                doc! {"$set": {"name":name.clone()}},
                None,
            )
            .await?;

        let mut classroom: ClassroomModel = ClassroomModel::new();

        for doc in cursor {
            classroom = bson::from_bson(Bson::Document(doc))?;
        }
        //return data
        match classroom._id.to_string() == "".to_string() {
            true => Err(FieldError::from("id not found")),
            false => Ok(classroom.to_norm()),
        }
    }

    // pub async fn delete_room(&self, ctx: &Context<'_>, id: String) -> FieldResult<String> {
    //     let db = ctx.data_unchecked::<AppContext>().db_pool.clone();
    //     let collection = db.database("rusttest").collection("classrooms");
    //     let converted_id = match bson::oid::ObjectId::parse_str(&id) {
    //         Ok(data) => data,
    //         Err(_) => return Err(FieldError::from("Not a valid id")),
    //     };
    //     let cursor = collection
    //         .delete_one(doc! { "_id": converted_id }, None)
    //         .await
    //         .unwrap();
    //     println!("{:?}", cursor.deleted_count);
    //     match cursor.deleted_count {
    //         1 => Ok(String::from("User deleted")),
    //         _ => Err(FieldError::from("User not delete")),
    //     }
    // }
}
