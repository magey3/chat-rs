use serde::{Serialize, Deserialize};
use diesel::Queryable;
use chrono::prelude::*;
use crate::schema::users;
use crate::schema::messages;

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
	pub name: &'a str,
	pub username: &'a str,
}

#[derive(Clone, Queryable, Deserialize, Serialize)]
pub struct User {
	pub id: i32,
	pub name: String,
	pub username: String,
}

#[derive(Serialize)]
pub struct JsonResponse {
	pub messages: Vec<Message>,
}

#[derive(Serialize)]
pub struct Message {
	pub content: String,
	pub user: User,
	pub time: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct ReceivedMessage {
	pub content: String,
	pub id: i32,
	pub time: DateTime<Utc>,
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
