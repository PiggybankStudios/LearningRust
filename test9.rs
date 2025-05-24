// TODO: We can't compile this with just the Rust compiler alone, we must use Crate which the book hasn't covered yet

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String
{
	return format!("Hello, {} year old named {}!", age, name);
}

fn main()
{
	rocket::ignite().mount("/", routes![hello]).launch();
}