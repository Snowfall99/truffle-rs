#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate rocket_contrib;

mod bft;

use bft::BFT;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[derive(Serialize)]
struct Context { 
    title: String,
    bfts: Vec<BFT>,
}

#[get("/")]
fn index() -> Template {
    let mut bfts = Vec::new();
    bfts.push(BFT{ id: 0, name: String::from("PBFT"), link: String::from("pbft.com") });
    bfts.push(BFT{ id: 1, name: String::from("Raft"), link: String::from("raft.com") });
    let context = Context{ title: String::from("Truffle Wiki"), bfts: bfts };
    Template::render("index", &context)
}

#[get("/bfts/<name>")]
fn bft(name: String) -> Template {
    let mut map = HashMap::new();
    map.insert("name", name);
    Template::render("bft", &map)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, bft])
        .attach(Template::fairing())
        .launch();
}
