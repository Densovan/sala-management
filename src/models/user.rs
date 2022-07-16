use serde::{Deserialize, Serialize};
use async_graphql::ID;
use bson::{self, doc, Document,Bson};

// Mongo Model
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserModel {
   pub id: ID,
   pub fullname: String,
   pub email:String,
   pub password:String,
   pub phone:String,
   pub gender:String,
}

//GQL Struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserGQL {
   pub id: ID,
   pub fullname: String,
   pub email:String,
   pub password:String,
   pub phone:String,
   pub gender:String,
}


//Base Implementation
impl UserGQL {
   pub fn new() -> Self {
      UserGQL { id: ID::from(""), fullname: String::from(""), email: String::from(""), password:String::from (""), phone:String::from(""), gender:String::from ("") }
   }
   pub fn to_bson_doc(&self) -> Document {
      // let converted_id = bson::oid::ObjectId::String(&self.id.to_string()).unwrap();
      let converted_id = Bson::String(self.id.to_string());
      doc! {
          "_id": converted_id,
          "email": self.email.to_owned(),
          "password": self.password.to_owned(),
      }
  }
}


//Mongo Implementation

impl UserModel {
    
}