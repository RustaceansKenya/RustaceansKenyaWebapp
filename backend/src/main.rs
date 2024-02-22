use rocket::fs::FileServer;

#[macro_use]
extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("assets/"))
        .mount("/hello", routes![hello])
}
