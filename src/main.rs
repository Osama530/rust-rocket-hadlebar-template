#![feature(decl_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;

use rocket::request::Form;
use rocket::Request;

use serde::Serialize;

#[derive(FromForm, Debug)]
struct Book {
  title : String,
  auther : String,
  isbn : String
}



#[post("/book", data = "<book_form>")]
fn new_book (book_form: Form<Book>)-> String {
  let book: Book = book_form.into_inner();
  let mut dummy_db: Vec<Book> = Vec::new();
  dummy_db.push(book);
  format!("book added succecfully.....{:?}",dummy_db)
}

#[get("/hello")]
fn hello() -> Json<&'static str> {
  Json("{
    'status': 'success',
    'message': 'Hello API!'
  }")
}

#[get("/")]
fn index()-> Template{

  #[derive(Serialize)]
  struct Context {
    first_name : String,
    last_name : String
  }
  
  let context = Context {
    first_name: String::from("osama"),
    last_name: String::from("Qamar")
  };

  Template::render("home", context) 
}

#[catch(404)]
fn not_found(req: &Request)-> String{
  format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

fn main() {
  rocket::ignite()
  .register(catchers![not_found])
  .mount("/api", routes![hello, new_book])
  .mount("/", routes![index])
  .attach(Template::fairing())
  .launch();
}