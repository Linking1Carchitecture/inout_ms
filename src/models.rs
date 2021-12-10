use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};
use self::super::schema::*;

extern crate diesel;

#[derive(Serialize, Deserialize, Identifiable, Debug, Queryable)]
pub struct User {
    id: i32,
    name: String,
    email: String,
    active: bool,
    sign_in_count: i64
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub active: bool,
    pub sign_in_count: i64
}

#[derive(Serialize, Deserialize, Debug, AsChangeset)]
#[table_name = "users"]
pub struct UpdateUser {
    pub name: String,
    pub email: String,
    pub active: bool,
    pub sign_in_count: i64
}

#[derive(Serialize, Deserialize, Identifiable, Debug, Queryable)]
pub struct Meet {
    id: i32,
    m_name: String,
    m_description: String,
    active: bool
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "meets"]
pub struct NewMeet {
    pub m_name: String,
    pub m_description: String,
    pub active: bool
}

#[derive(Serialize, Deserialize, Debug, AsChangeset)]
#[table_name = "meets"]
pub struct UpdateMeet {
    pub m_name: String,
    pub m_description: String,
    pub active: bool
}