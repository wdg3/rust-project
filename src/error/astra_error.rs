use axum::http::StatusCode;
use std::{error::Error, fmt};

#[derive(Debug)]
pub struct AstraError { 
    code: StatusCode,
    message: String,
}

impl fmt::Display for AstraError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", &self.code, &self.message)
    }
}

impl Error for AstraError {
    fn description(&self) -> &str {
        "Astra service error"
    }
}