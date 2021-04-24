#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::{NamedFile, content};
use std::path::{Path, PathBuf};
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use rocket::response::content::Html;
use std::{env, fs};


mod api_data_types;
mod api_wrappers;

// #[get("/")]
// fn index() -> Template {
//     // let context = context();
//     let context: HashMap<&str, &str> = HashMap::new();
//     Template::render("request_page", &context)
// }
#[get("/")]
fn index() -> content::Html<String> {
    // let cur_dir = env::current_dir().unwrap();
    // println!("{:?}", cur_dir);
    // for d in fs::read_dir(cur_dir).unwrap() {
    //     println!("{:?}", d)
    // }
    // let context = context();
    let mut html_content = String::new();
    let request_page_file = File::open("templates/request_page.html").unwrap();
    let mut buf_reader = BufReader::new(request_page_file);
    buf_reader.read_to_string(&mut html_content);
    content::Html(html_content)
}

#[get("/templates/<resource..>")]
fn templates(resource: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("templates").join(resource)).ok()
}

// #[post("/new_request", format = "multipart/form-data", data = "<request>")]
// fn new_request(request: api_data_types::Request) -> &'static str {
//     /*TODO: write REQUEST type*/
//     "downloadable"
// }
//
//
// #[post("/get_status", format = "application/json", data = "<request_id>")]
// fn get_status(request_id: u32) {
//     println!("аыа");
// }


#[get("/downloads/<request_id>")]
fn download_result(request_id: u32) -> Option<NamedFile> {
    // NamedFile::open(Path::new(String::from("static/")+request_id.to_string())).ok()
    NamedFile::open(Path::new(&format!("static/{}", request_id.to_string()))).ok()
}


fn main() {
    rocket::ignite()
        .mount("/", routes![index, templates])
        // .mount("/", routes![index])
        .attach(Template::fairing())
        .launch();

    // rocket::ignite().mount("/api", routes!(new_request)).launch();
}