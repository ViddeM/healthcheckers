mod dashboard;
use dashboard::get_dashboard;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_dashboard])
        .mount("/api/public", FileServer::from("static/public"))
        .attach(Template::fairing())
}
