#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn main() {
    let _ = rocket::build().mount("/", routes![index]).launch().await;
}