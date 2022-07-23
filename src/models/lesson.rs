use async_graphql::ID;
use serde::{Deserialize, Serialize};
use bson::{self, doc, oid::ObjectId, Document, Bson};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonModel {
    pub _id: ObjectId,
    pub userId: String,
    pub les: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonGQL {
    pub id: ID,
    pub room_id: String,
    pub lesson_id: String,
}

impl LessonGQL {
    pub fn new() -> Self {
        LessonGQL {
            id: ID::from(""),
            room_id: String::from(""),
            lesson_id: String::from(""),
        }
    }

    pub fn to_bson_doc(&self) -> Document {
        let converted_id = Bson::String(self.id.to_string());
        doc! {
            "_id": converted_id,
            "room_id": self.room_id.to_owned(),
            "lesson_id": self.lesson_id.to_owned(),
        }
    }
}

impl LessonModel {
    pub fn new() -> LessonModel {
        LessonModel {
            _id: bson::oid::ObjectId::new(),
            room_id: String::from(""),
            lesson_id: String::from(""),
        }
    }

    pub fn to_norm(&self) -> LessonGQL {
        LessonGQL {
            id: ID::from(self._id.to_string()),
            room_id: self.room_id.to_owned(),
            lesson_id: self.lesson_id.to_owned(),
        }
    }
}

#[async_graphql::Object]
impl SessionGQL {
    async fn id(&self) -> &str {
        &self.id
    }
    async fn room_id(&self) -> &str {
        &self.room_id
    }
    async fn lesson_id(&self) -> &str {
        &self.lesson_id
    }
}