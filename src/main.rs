#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::{io, env};
use rocket::{Request, Data, response::Debug};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/captions", format = "plain", data = "<data>")]
fn captions(data: Data) -> Result<String, Debug<io::Error>> {
    Result::Ok(String::from("Ok"))
}

#[catch(404)]
fn not_found(request: &Request) -> String {
    let format = request.format().unwrap();
    let s = format!("{}\n", format);
    return s;
}

fn main() {
    rocket::ignite().mount("/", routes![index])
        .register(catchers![not_found])
        .launch();
}