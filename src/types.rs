use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};



#[derive(Serialize,Deserialize)]
pub struct User {
    pub _id: Option<ObjectId>,
    pub full_name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub gender: Option<String>,
    pub phone: Option<String>,
}
