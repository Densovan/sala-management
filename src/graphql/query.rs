// Library imports
use async_graphql::{Context, FieldError, FieldResult, Object};
use bson;
#[allow(unused_imports)]
use futures::{lock::Mutex, stream::StreamExt};
use mongodb::bson::{doc, Bson};

// MODELS
use super::AppContext;
use crate::models::user::{UserGQL, UserModel};
pub struct RootQuery;

#[Object]
impl RootQuery {
    // =====================>>user section <<====================

    pub async fn users(&self, ctx: &Context<'_>) -> FieldResult<Vec<UserGQL>> {
        let db = ctx.data_unchecked::<AppContext>().db_pool.clone();
        let collection = db.database("rusttest").collection("users");
        let mut data: Vec<UserGQL> = Vec::new();
        let mut cursor = collection.find(None, None).await?;

        // Iterate over the results of the cursor.
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    let u: UserModel = bson::from_bson(Bson::Document(document))?;
                    data.push(u.to_norm());
                }
                Err(e) => return Err(e.into()),
            }
        }

        match data.is_empty() {
            true => Ok(data),
            false => Ok(data),
        }
    }

    // get user by ID public

    pub async fn user_by_id(&self, ctx: &Context<'_>, id: String) -> FieldResult<UserGQL> {
        let db = ctx.data_unchecked::<AppContext>().db_pool.clone();
        let collection = db.database("rusttest").collection("users");

        let converted_id = match bson::oid::ObjectId::parse_str(&id) {
            Ok(data) => data,
            Err(_) => return Err(FieldError::from("Not a valid id")),
        };

        //create query

        let cursor = collection
            .find_one(doc! {"_id": converted_id}, None)
            .await
            .unwrap_or(None);

        let mut user: UserModel = UserModel::new();

        for doc in cursor {
            user = bson::from_bson(Bson::Document(doc))?;
        }

        //return data
        match user._id.to_string() == "".to_string() {
            true => Err(FieldError::from("User not found")),
            false => Ok(user.to_norm()),
        }
    }

    //user by email

    pub async fn user_by_email(&self, ctx: &Context<'_>, email: String) -> FieldResult<UserGQL> {
        let db = ctx.data_unchecked::<AppContext>().db_pool.clone();
        let collection = db.database("rusttest").collection("users");

        let cursor = collection
            .find_one(doc! {"email": email}, None)
            .await
            .unwrap_or(None);

        let mut user: UserModel = UserModel::new();

        for doc in cursor {
            user = bson::from_bson(Bson::Document(doc))?;
        }

        //return data
        match user.email.to_string() == "".to_string() {
            true => Err(FieldError::from("User not found")),
            false => Ok(user.to_norm()),
        }
    }
}
