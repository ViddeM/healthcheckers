#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/api", routes![health])
}

#[get("/health?<state>")]
fn health(state: Option<String>) -> String {
    state.unwrap_or_default()
}
