extern crate actix_web;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate failure;

use actix_web::{
    App, 
    server, 
    HttpRequest, 
    http::{Method, StatusCode}, 
    fs, 
    Result, 
    Json, 
    middleware::cors::Cors,
    Path};
use std::path::PathBuf;

mod error;

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

#[derive(Deserialize)]
struct BigPath {
    name: String,
    partner: String,
    friend: String,
    pet: String,
    age: u32,
    boolean: bool,
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

fn easy_parameters_reciver(path: Path<BigPath>) -> Result<Json<HelloResponse>> {
    let id: u32 = if path.boolean {
        path.age * 2
    } else {
        path.age
    };
    Ok(Json (
        HelloResponse {
            greetings: format!("Hi {} with partner {} and friends {}, {}",path.name,path.partner,path.friend,path.pet),
            id: id,
        }
    ))
}

fn handler_404(_req: &HttpRequest) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

fn main() {
    server::new(move || {
        App::new()
            .prefix("api")
            .configure( | app | {
                Cors::for_app(app)
                    .allowed_origin("http://localhost:3000")
                      .max_age(3600)
            .resource("/",|r|  {
                    r.method(Method::GET).f(index_get);
                    r.method(Method::POST).with(index_post);
            })
                .resource(r"/greetings/{name}/{age}", |r| r.method(Method::POST).f(greetings_url))
                .resource(r"/infinite/{list:.*}", |r| r.method(Method::POST).f(infinite_parameters_url))
                .resource(r"/path/{name}/{partner}/{friend}/{pet}/{boolean}/{age}", |r| r.method(Method::POST).with(easy_parameters_reciver))
                .resource("/get", |r| r.method(Method::GET).f(index_get))
                .resource("/post", |r| r.method(Method::POST).with(index_post))
                .resource("/error", |r| r.method(Method::POST).f(error::func_with_error))
                .register()
            })
            .default_resource(|r| r.f(handler_404))
   })
    .bind("127.0.0.1:8000")
    .unwrap()
    .run()
}