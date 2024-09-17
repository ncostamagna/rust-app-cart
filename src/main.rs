use log::{info, warn};
/*#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}*/

use std::process::ExitCode;
mod logger;

#[rocket::main]
async fn main() -> ExitCode {
    logger::init();

    info!("starting up");

    warn!("shutting down");


    ExitCode::SUCCESS
}
