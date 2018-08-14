extern crate actix_web;

use actix_web::{App, server, HttpRequest, http::{Method, StatusCode}, fs, Result};

fn index_get(_req: &HttpRequest) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/index.html")?)
}


fn index_post(_req: &HttpRequest) -> &'static str {
    "Hola peticion post"
}

fn handler_404(_req: &HttpRequest) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
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
            .default_resource(|r| r.f(handler_404))
    })
    .bind("127.0.0.1:8000")
    .unwrap()
    .run()
}