#![feature(proc_macro_hygiene, decl_macro)]
use rocket::*;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;

mod schema;
mod models;


#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
	format!("Hello {} year old named {}!", age, name)
}

#[get("/porfolio/api/all")]
fn all() -> String {
	format!(models::Book::all(&conn))
}

fn main() {
	dotenv().ok();

	let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
	let conn = PgConnection::establish(&database_url).unwrap();

	let book = models::NewBook {
		title: String::from("Gravity's Rainbow"),
		author: String::from("Thomas Pynchon"),
		published: true,
	};

	if models::Book::insert(book, &conn) {
		println!("success");
	} else {
		println!("failed");
	}

	// rocket::ignite().mount("/", routes![hello]).launch();
}
