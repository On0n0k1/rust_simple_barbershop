use std::fmt::{self};

use actix_web::{http::StatusCode, HttpResponse, ResponseError};

#[derive(Debug)]
pub enum Error {
    Sqlx(sqlx::Error),
}

impl Error {
    pub fn sqlx<S>(err: sqlx::Error) -> Result<S, Self> {
        Err(Self::Sqlx(err))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match &self {
            Self::Sqlx(_err) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let msg = match &self {
            Self::Sqlx(err) => format!("A database error has occurred: {err:?}"),
        };
        HttpResponse::build(self.status_code()).body(msg)
    }
}
