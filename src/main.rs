#![feature(plugin, const_fn, proc_macro_hygiene, decl_macro)]
use rocket::*;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;

mod schemas;
mod models;
mod db;
mod routes;
use crate::routes::portfolio::*;


fn rocket() -> rocket::Rocket {
	dotenv().ok();

	let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");

	let pool = db::init_pool(database_url);
	rocket::ignite()
		.manage(pool)
		.mount(
			"/api/v1/", 
			routes![index, new, show, delete, author, update],
		)
		.register(catchers![not_found])
}

fn main() {
	rocket().launch();
}
