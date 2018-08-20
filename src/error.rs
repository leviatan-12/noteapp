use actix_web::{HttpRequest, error};
use failure::Error;

#[derive(Fail, Debug)]
#[fail(display="my error")]
pub struct MiError {
    name: &'static str
}

impl error::ResponseError for MiError {}

pub fn func_with_error(_req: &HttpRequest) -> Result<&'static str,MiError> {
    Err(MiError{name:"prueba"})
}