use log::{info, warn, error};
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
mod router;
mod cart;

#[rocket::main]
async fn main() -> ExitCode {
    crate::logger::init();

    info!("starting up");
    router::putapario();
    /*let exit = match crate::router2::init2().ignite().await {
        Ok(rocket) => {
            let listen = format!("{}:{}", rocket.config().address, rocket.config().port);

            info!("listening");

            match rocket.launch().await {
                Ok(_) => ExitCode::SUCCESS,
                Err(err) => {
                    error!("rocket launch failed with {err}");
                    ExitCode::FAILURE
                }
            }
        }
        Err(err) => {
            error!("rocket ignite failed with {err}");
            ExitCode::FAILURE
        }
    };*/

    warn!("shutting down");

    ExitCode::SUCCESS
  //  exit
}
