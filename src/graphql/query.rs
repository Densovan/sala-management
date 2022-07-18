// Library imports
use async_graphql::{Context, FieldResult, Object};
use bson;
#[allow(unused_imports)]
use futures::{lock::Mutex, stream::StreamExt};
use mongodb::bson::Bson;

// MODELS
use super::AppContext;
use crate::models::user::{UserGQL, UserModel};
pub struct RootQuery;

#[Object]
impl RootQuery {
    pub async fn users(&self, ctx: &Context<'_>) -> FieldResult<Vec<UserGQL>> {
        let db = ctx.data_unchecked::<AppContext>().db_pool.to_owned();
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

    async fn value(&self) -> i32 {
        // A GraphQL Object type must define one or more fields.
        100
    }
}
