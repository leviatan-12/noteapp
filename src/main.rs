extern crate actix_web;

#[macro_use]
extern crate serde_derive;

use actix_web::{App, server, HttpRequest, http::{Method, StatusCode}, fs, Result, Json, Path};
use std::path::PathBuf;

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

fn greetings_url(req: &HttpRequest) -> Result<Json<HelloResponse>> {
    let name: String = req.match_info().query("name")?;
    let age: u32 = req.match_info().query("age")?;
    Ok(Json(
        HelloResponse{
            greetings: format!("Bonjour {}",name),
            id: age*2,
        }
    ))
}

fn infinite_parameters_url(req: &HttpRequest) -> Result<Json<HelloResponse>>{
    let path: PathBuf = req.match_info().query("list")?;
    Ok(Json(
        HelloResponse {
            greetings: format!("List: {:?}",path),
            id: 0
        }
    ))
}

fn easy_parameters_reciver(path: Path<(String,u32)>) -> Result<Json<HelloResponse>> {
    Ok(Json (
        HelloResponse {
            greetings: format!("Hola {}",path.0),
            id: path.1*32,
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
            .resource(r"/greetings/{name}/{age}", |r| r.method(Method::POST).f(greetings_url))
            .resource(r"/infinite/{list:.*}", |r| r.method(Method::POST).f(infinite_parameters_url))
            .resource(r"/path/{name}/{age}", |r| r.method(Method::POST).with(easy_parameters_reciver))
            .resource("/get", |r| r.method(Method::GET).f(index_get))
            .resource("/post", |r| r.method(Method::POST).with(index_post))
            .default_resource(|r| r.f(handler_404))
    })
    .bind("127.0.0.1:8000")
    .unwrap()
    .run()
}