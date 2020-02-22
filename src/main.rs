#![feature(plugin, const_fn, proc_macro_hygiene, decl_macro)]
use rocket::*;
use rocket_cors;
use rocket::http::Method; // 1

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
use routes::*;

use rocket_cors::{
    AllowedHeaders, AllowedOrigins, Error, // 2.
    Cors, CorsOptions // 3.
};


fn make_cors() -> Cors {	
	dotenv().ok();

	// WILL BE LIST OF ADDRESS ROOTS. UPDATE THIS WHEN I KNOW HOW TO MAP LIST OF ENVS AS ARRAY
	let portfolio_address = env::var("PORTFOLIO_ROOT_ADDRESS").expect("set PORTFOLIO_ROOT_ADDRESS");
    
    let allowed_origins = AllowedOrigins::some_exact(&[ // 4.
        portfolio_address,
    ]);

    CorsOptions { // 5.
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(), // 1.
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin", // 6.
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}

fn rocket() -> rocket::Rocket {
	dotenv().ok();

	let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");

	let pool = db::init_pool(database_url);
	rocket::ignite()
		.manage(pool)
		.mount(
			"/portfolio/api/v1/", 
			routes![
				portfolio::index, 
				portfolio::new,  
				portfolio::delete, 
				portfolio::update
			],
		)
		.register(catchers![portfolio::not_found])
		.attach(make_cors())
}

fn main() -> Result<(), Error> {
	rocket().launch();

	Ok(())
}
