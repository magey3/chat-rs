#[macro_use]
extern crate diesel;

mod data;
mod schema;

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

	//diesel::insert_into(users)
	//	.values(NewUser {name: "John Doe", username: "lomox_123"})
	//	.execute(&c)
	//	.expect("Error saving new post");

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
			user: users
				.filter(crate::schema::users::dsl::id.eq(i.userid))
				.load::<User>(&c)
				.expect("Failed to get user")[0].clone(),
			time: Utc.from_utc_datetime(&dt_tz.naive_utc())
		});
	}

	HttpResponse::Ok().json(out)
}

#[post("/")]
async fn send_message(_item: web::Form<Message>) -> impl Responder {
	HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let mut ssl = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
	ssl.set_private_key_file("../key.pem", SslFiletype::PEM).unwrap();
	ssl.set_certificate_chain_file("../cert.pem").unwrap();

	HttpServer::new(|| {
		App::new()
			.service(get_messages)
			.service(Files::new("/", "./static/").index_file("index.html"))
	})
		.bind_openssl("127.0.0.1:8080", ssl)?
		.run()
		.await
}
