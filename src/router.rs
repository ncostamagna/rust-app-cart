/*!
Rocket app configuration.
*/

use crate::domain;
use rocket::Build;


pub fn init() -> rocket::Rocket<Build> {
    domain::controller::hijodeputa();
    rocket::build().mount(
            "/carts",
            rocket::routes![domain::controller::get],
        )
}

pub fn putapario() {
    println!("putapario");
}