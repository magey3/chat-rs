use serde::{Serialize, Deserialize};
use diesel::Queryable;
use chrono::prelude::*;
use crate::schema::users;
use crate::schema::messages;

#[derive(Insertable, Deserialize)]
#[table_name="users"]
pub struct NewUser {
	pub name: String,
	pub username: String,
	pub password: String,
	pub email: String,
}

#[derive(Clone, Queryable, Deserialize, Serialize)]
pub struct SqlUser {
	pub id: i32,
	pub name: String,
	pub username: String,
	pub password: String,
	pub email: String,
}

#[derive(Deserialize)]
pub struct LoginInfo {
	pub email: String,
	pub password: String,
}

#[derive(Serialize)]
pub struct JsonResponse {
	pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize)]
pub struct UserReference {
	pub id: i32,
	pub name: String,
	pub username: String,
}
impl UserReference {
	pub fn from(x: SqlUser) -> UserReference {
		UserReference {id: x.id, name: x.name, username: x.username}
	}
}

#[derive(Serialize)]
pub struct Message {
	pub content: String,
	pub user: UserReference,
	pub time: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct ReceivedMessage {
	pub content: String,
	pub id: i32,
	pub time: DateTime<Utc>,
	pub password: String,
}

#[derive(Queryable)]
pub struct SqlMessage {
	pub id: i32,
	pub content: String,
	pub userid: i32,
	pub time: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="messages"]
pub struct InsertSqlMessage {
	pub content: String,
	pub userid: i32,
	pub time: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct MessageRequest {
	pub index: i64,
	pub amount: i64,
}
