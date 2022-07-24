use async_graphql::ID;
use bson::{doc, oid::ObjectId, Bson, Document};
// use chrono::*;
use serde::{Deserialize, Serialize};

//========================Mongo Model============================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassroomModel {
    pub _id: ObjectId,
    pub school_id: String,
    pub name: String,
    pub date: bson::DateTime,
}

//============================**GQL Struct**==============================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassroomGQL {
    pub id: ID,
    pub school_id: String,
    pub name: String,
    pub date: String,
    pub message: String,
}

//=========================**Base Implementation**==================================
impl ClassroomGQL {
    pub fn _new() -> Self {
        ClassroomGQL {
            id: ID::from(""),
            school_id: String::from(""),
            name: String::from(""),
            date: String::from(""),
            message: String::from(""),
        }
    }
    pub fn _to_bson_doc(&self) -> Document {
        let converted_id = Bson::String(self.id.to_string());
        doc! {
            "_id": converted_id,
            "name":self.name.to_owned(),
            "date":self.date.to_owned(),
        }
    }
}

impl ClassroomModel {
    pub fn new() -> Self {
        ClassroomModel {
            _id: bson::oid::ObjectId::new(),
            school_id: String::from(""),
            name: String::from(""),
            date: bson::DateTime::now(),
        }
    }

    pub fn to_norm(&self) -> ClassroomGQL {
        ClassroomGQL {
            id: ID::from(self._id.to_string()),
            school_id: self.school_id.to_owned(),
            name: self.name.to_owned(),
            date: self.date.to_owned().to_string(),
            message: String::from(""),
        }
    }
}

//======================**GraphQL**=========================
#[async_graphql::Object]
impl ClassroomGQL {
    async fn id(&self) -> &str {
        &self.id
    }
    async fn name(&self) -> &str {
        &self.name
    }
    async fn date(&self) -> &str {
        &self.date
    }
    async fn school_id(&self) -> &str {
        &self.school_id
    }
    async fn message(&self) -> &str {
        &&self.message
    }
}
