use async_graphql::ID;
use bson::Bson;
use bson::{self, doc, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};
use validator::Validate;

// ============================**Mongo Model**==============================
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Validate)]

pub struct UserModel {
    pub _id: ObjectId,
    pub fullname: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub phone: String,
    pub gender: String,
}

//============================**GQL Struct**==============================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserGQL {
    pub id: ID,
    pub fullname: String,
    pub email: String,
    pub password: String,
    pub phone: String,
    pub gender: String,
}

//=========================**Base Implementation**==================================
impl UserGQL {
    pub fn _new() -> Self {
        UserGQL {
            id: ID::from(""),
            fullname: String::from(""),
            email: String::from(""),
            password: String::from(""),
            phone: String::from(""),
            gender: String::from(""),
        }
    }
    pub fn _to_bson_doc(&self) -> Document {
        // let converted_id = bson::oid::ObjectId::with_string(&self.id.to_string()).unwrap();

        let converted_id = Bson::String(self.id.to_string());
        doc! {
            "_id": converted_id,
            "email": self.email.to_owned(),
            "password": self.password.to_owned(),
        }
    }
}

//============================**Mongo Implementation**=========================
impl UserModel {
    pub fn new() -> UserModel {
        UserModel {
            _id: bson::oid::ObjectId::new(),
            fullname: String::from(""),
            email: String::from(""),
            password: String::from(""),
            phone: String::from(""),
            gender: String::from(""),
        }
    }

    pub fn to_norm(&self) -> UserGQL {
        UserGQL {
            id: ID::from(self._id.to_string()),
            fullname: self.fullname.to_owned(),
            email: self.email.to_owned(),
            password: self.password.to_owned(),
            phone: self.phone.to_owned(),
            gender: self.gender.to_owned(),
        }
    }
}

//======================**GraphQL**=========================
#[async_graphql::Object]
impl UserGQL {
    async fn id(&self) -> &str {
        &self.id
    }
    async fn email(&self) -> &str {
        &self.email
    }
    async fn password(&self) -> &str {
        &self.password
    }
    async fn fullname(&self) -> &str {
        &self.fullname
    }
    async fn phone(&self) -> &str {
        &self.phone
    }
    async fn gender(&self) -> &str {
        &self.gender
    }
}
