extern crate actix_web;

#[macro_use]
extern crate serde_derive;

use actix_web::{App, server, HttpRequest, http::{Method, StatusCode}, fs, Result, Json};

#[derive(Serialize)]
struct HelloResponse {
    greetings: String,
    id: u32,
}

#[derive(Deserialize)]
struct HelloRequest {
    name: String,
    age: u32
}

fn index_get(_req: &HttpRequest) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/index.html")?)
}


fn index_post(req: Json<HelloRequest>) -> Result<Json<HelloResponse>> {
    Ok(Json(
        HelloResponse{
            greetings: format!("Hi {}",req.name),
            id: (req.age*2),
        }
    ))
}

fn handler_404(_req: &HttpRequest) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

fn main() {
    server::new(move || {
        App::new()
            .resource("/",|r|  {
                    r.method(Method::GET).f(index_get);
                    r.method(Method::POST).with(index_post);
            })

            .resource("/get", |r| r.method(Method::GET).f(index_get))
            .resource("/post", |r| r.method(Method::POST).with(index_post))
            .default_resource(|r| r.f(handler_404))
    })
    .bind("127.0.0.1:8000")
    .unwrap()
    .run()
}