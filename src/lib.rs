#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern  crate diesel;
extern crate dotenv;
extern crate serde_json;
extern crate rocket;
extern crate rocket_contrib;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use self::models::{BFT, NewBFT};

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_bft<'a>(conn: &MysqlConnection, name: &str, link: &str) -> BFT {
    use schema::bfts;

    let new_bft = NewBFT{ name, link };
    diesel::insert_into(bfts::table)
        .values(&new_bft)
        .execute(conn)
        .expect("Error saving new bft");
    bfts::table.order(bfts::id.desc()).first(conn).unwrap()
}
