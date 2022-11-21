mod hello;

use actix_web::{App};
use hello::hello;


pub fn services_init(app: &App<AppEntry>) -> App<AppEntry> {
    app.service(hello)
}