use blogger::db;
use rocket_db_pools::Database;

#[macro_use]
extern crate rocket;
#[launch]
fn entry() -> _ {
    rocket::build()
        .attach(db::BloggerDatabase::init())
        .mount("/", routes![index])
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
