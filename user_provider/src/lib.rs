use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub struct UserIn {
    pub username: String,
    pub name: String,
    pub surname: String,
    pub password: String,
    pub age: i32,
    pub job_title: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub struct UserOut {
    pub id: Uuid,
    pub username: String,
    pub name: String,
    pub surname: String,
    pub age: i32,
    pub timestamp: NaiveDateTime,
    pub job_title: Option<String>,
    pub phone_number: Option<String>,
}
