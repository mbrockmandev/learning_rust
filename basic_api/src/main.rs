use handlers::get_counter::get_counter;
use rocket::routes;
use std::io::Error;

fn main() -> Result<(), Error> {
    rocket::build()
        .mount("/id/count", routes![get_counter])
        .launch();
    Ok(())
}
