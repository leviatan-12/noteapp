extern crate actix_web;

use actix_web::{App, server, HttpRequest, http::{Method}, HttpResponse};

fn index_get(_req: &HttpRequest) -> &'static str {
    "Hola peticion get"
}


fn index_post(_req: &HttpRequest) -> &'static str {
    "Hola peticion post"
}

fn main() {
    server::new(move || {
        App::new()
            .resource("/",|r|  {
                    r.method(Method::GET).f(index_get);
                    r.method(Method::POST).f(index_post);
            })

            .resource("/get", |r| r.method(Method::GET).f(index_get))
            .resource("/post", |r| r.method(Method::POST).f(index_post))
    })
    .bind("127.0.0.1:8000")
    .unwrap()
    .run()
}