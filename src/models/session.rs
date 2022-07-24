use async_graphql::ID;
use bson::{self, doc, oid::ObjectId, Bson, Document};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionModel {
    pub _id: ObjectId,
    pub room_id: String,
    pub lesson_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionGQL {
    pub id: ID,
    pub room_id: String,
    pub lesson_id: String,
}

impl SessionGQL {
    pub fn _new() -> Self {
        SessionGQL {
            id: ID::from(""),
            room_id: String::from(""),
            lesson_id: String::from(""),
        }
    }

    pub fn _to_bson_doc(&self) -> Document {
        let converted_id = Bson::String(self.id.to_string());
        doc! {
            "_id": converted_id,
            "room_id": self.room_id.to_owned(),
            "lesson_id": self.lesson_id.to_owned(),
        }
    }
}

impl SessionModel {
    pub fn _new() -> SessionModel {
        SessionModel {
            _id: bson::oid::ObjectId::new(),
            room_id: String::from(""),
            lesson_id: String::from(""),
        }
    }

    pub fn _to_norm(&self) -> SessionGQL {
        SessionGQL {
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
