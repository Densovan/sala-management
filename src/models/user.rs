use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]

pub struct User {
   pub fullname: String,
   pub email:String,
   pub phone:String,
   pub gender:String,
}