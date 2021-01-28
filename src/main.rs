#[macro_use]
extern crate diesel;

mod data;
mod schema;
mod db;

use actix_files::Files;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use crate::data::*;
use chrono::prelude::*;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[post("/json")]
async fn get_messages(item: web::Json<MessageRequest>) -> impl Responder {
	use crate::schema::users::dsl::*;
	use crate::schema::messages::dsl::*;

	dotenv().ok();

	let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	let c = PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url));

	let message_v = messages
		.limit(item.amount)
		.load::<SqlMessage>(&c)
		.expect("Error loading messages");

	let mut out: JsonResponse = JsonResponse {messages: Vec::new()}; 
	for i in message_v {
		let tz = FixedOffset::east(0);
		let dt_tz = tz.from_local_datetime(&i.time).unwrap();
		out.messages.push(Message {
			content: i.content,
			user: UserReference::from(users
				.filter(crate::schema::users::dsl::id.eq(i.userid))
				.load::<SqlUser>(&c)
				.expect("Failed to get user")[0].to_owned()),
			time: Utc.from_utc_datetime(&dt_tz.naive_utc())
		});
	}

	HttpResponse::Ok().json(out)
}

#[post("/")]
async fn send_message(item: web::Form<ReceivedMessage>) -> impl Responder {

	use crate::schema::messages::dsl::*;
	dotenv().ok();

	let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	let c = PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url));

	//TODO check if the user exists
	diesel::insert_into(messages)
		.values(InsertSqlMessage {content: item.content.to_owned(), time: Utc::now().naive_utc(), userid: item.id})
		.execute(&c)
		.expect("Error saving new post");
		
	HttpResponse::Ok()
}

#[post("/register")]
async fn register(item: web::Form<NewUser>) -> impl Responder{
	dotenv().ok();

	let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	let c = PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url));
	
	HttpResponse::Ok().json(UserReference::from(crate::db::hashed_insert_user(item.into_inner(), &c).expect("Error inserting new user")))
}

#[post("/login")]
async fn login(item: web::Form<LoginInfo>) -> impl Responder{
	dotenv().ok();

	let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	let c = PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url));
	
	let user = crate::db::authorize(&item.password, &item.email, &c);

	HttpResponse::Ok().json(UserReference::from(user))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let mut ssl = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
	ssl.set_private_key_file("../key.pem", SslFiletype::PEM).unwrap();
	ssl.set_certificate_chain_file("../cert.pem").unwrap();

	HttpServer::new(|| {
		App::new()
			.service(get_messages)
			.service(send_message)
			.service(login)
			.service(register)
			.service(Files::new("/", "./static/").index_file("index.html"))
	})
		.bind_openssl("127.0.0.1:8080", ssl)?
		.run()
		.await
}
