#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::NamedFile;
use std::path::Path;

mod api_data_types;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/new_request", format = " multipart/form-data", data = "<request>")]
fn new_request(request: api_data_types::Request) -> &'static str {
    /*TODO: write REQUEST type*/
    "downloadable"
}


#[post("/get_status", format = "application/json", data = "<request_id>")]
fn get_status(request_id: u32) {
    println!("аыа");
}


#[get("/downloads/<request_id>")]
fn download_result(request_id: u32) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/"+request_id)).ok()
}


fn main() {
    rocket::ignite().mount("/", routes![index]).launch();

    rocket::ignite().mount("/api", routes!(new_request)).launch();
}