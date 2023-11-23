use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize,  Validate)]
pub struct SignUser {

    #[validate(email)]
    pub name: String,

    #[validate(length(min = 6, max = 20))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Option<i32>,
    pub uuid: String,
    pub name: String,
    pub password: String,
    pub status: i8,
    pub create_time: Option<DateTime<Utc>>,
    pub update_time: Option<DateTime<Utc>>,
}

