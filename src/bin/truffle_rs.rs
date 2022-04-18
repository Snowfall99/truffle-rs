#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate serde_derive;
extern crate diesel;
extern crate serde_json;
extern crate rocket_contrib;
extern crate truffle_rocket;

use diesel::prelude::*;
use rocket::Request;
use truffle_rocket::{models::*, establish_connection, create_bft};
use rocket_contrib::templates::Template;
use rocket_contrib::json::Json;

#[get("/")]
fn index() -> Template {
    use truffle_rocket::schema::bfts::dsl::*;

    let conn = establish_connection();
    let results = bfts
        .load::<BFT>(&conn)
        .expect("Error loading bfts");
    println!("Displaying {} bfts", results.len());
    let b = BFTS{bfts: results};
    Template::render("index", &b)
}

#[post("/bfts", data="<bft>")]
fn write_bft(bft: Json<NewBFT>) -> String {
    let conn = establish_connection();

    let name = bft.0.name;
    let link = bft.0.link;

    let bft = create_bft(&conn, &name, &link);
    format!("create {} with id {}", bft.name, bft.id)
}

#[get("/bfts/<n>")]
fn get_bft(n: String) -> Template {
    use truffle_rocket::schema::bfts::dsl::*;

    let conn = establish_connection();
    let result = bfts
        .filter(name.eq(n))
        .load::<BFT>(&conn)
        .expect("Error loading bft");
    let b = result[0].clone();
    Template::render("bft", &b)
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, {} is not a valid path.", req.uri())
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, write_bft, get_bft])
        .register(catchers![not_found])
        .attach(Template::fairing())
        .launch();
}
