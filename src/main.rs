extern crate actix_web;

use actix_web::{App, server, HttpRequest};

fn index(_req: &HttpRequest) -> &'static str {
    "Hola mundo"
}

fn main() {
    server::new(move || {
        App::new()
            .resource("/",|r| r.f(index))
    })
    .bind("127.0.0.1:8000")
    .unwrap()
    .run()
}