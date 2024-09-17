/*!
Rocket app configuration.
*/

use crate::cart;
use rocket::Build;


pub fn init() -> rocket::Rocket<Build> {
    cart::controller::hijodeputa();
    rocket::build().mount(
            "/carts",
            rocket::routes![cart::controller::get],
        )
}

pub fn putapario() {
    println!("putapario");
}