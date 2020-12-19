use std::io;
use rocket::response::{NamedFile}; // 1.

#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html") // 2.
}