use rocket::{
    http::{uri::Origin, Status},
    response::Redirect,
    serde::json::{json, Value},
};

#[macro_use]
extern crate rocket;

const TAURI_RELEASES_PREFIX: Origin<'static> = uri!("/tauri-releases");

#[get("/")]
fn index() -> Redirect {
    let msg: Option<&str> = None;
    Redirect::to(uri!(
        TAURI_RELEASES_PREFIX,
        google_keep_desktop_api("google_keep_desktop_api", "1.0.14", msg)
    ))
}

#[get("/google-key-desktop/<_platform>/<current_version>?<msg>")] // &<other_stuff>
fn google_keep_desktop_api(_platform: &str, current_version: &str, msg: Option<&str>) -> Status {
    Status::NoContent
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount(TAURI_RELEASES_PREFIX, routes![google_keep_desktop_api])
}
