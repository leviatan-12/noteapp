use actix_web::{HttpRequest, error, Result};

#[derive(Fail, Debug)]
#[fail(display="my error")]
pub struct MiError {
    name: &'static str
}

impl error::ResponseError for MiError {}

pub fn func_with_error(_req: &HttpRequest) -> Result<&'static str> {
    let response: Result<&'static str, MiError> = Err(MiError{name:"prueba"});

    Ok(response.map_err(|e| error::ErrorBadRequest(e.name))?)
}