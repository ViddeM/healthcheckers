mod dashboard;
use config::Config;
use dashboard::get_dashboard;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

#[macro_use]
extern crate rocket;

mod config;

#[launch]
async fn rocket() -> _ {
    let config = Config::new().expect("Failed to read config");

    rocket::build()
        .mount("/", routes![get_dashboard])
        .mount("/api/public", FileServer::from("static/public"))
        .manage(config)
        .attach(Template::fairing())
}
