/*
//  Copyright 2021 Catgirl.Moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use actix_http::http::StatusCode;
use actix_web::{error::ResponseError, HttpResponse};
use askama::Error as AskamaError;
use std::io::Error as IOError;
use std::{convert::From, fmt::Debug};
use derive_more::Display; //TODO: Ditch single use dependency


#[derive(Debug, Display)]
pub enum WebError {
  NotFound,
  AskamaError(String),
  IOError(String)
}

impl ResponseError for WebError {
  fn error_response(&self) -> HttpResponse {
    match self {
      WebError::NotFound => HttpResponse::BadRequest().status(StatusCode::NOT_FOUND).finish(),
      _ => HttpResponse::BadRequest().status(StatusCode::INTERNAL_SERVER_ERROR).finish()
    }
  }
}

impl From<AskamaError> for WebError {
  fn from(err: AskamaError) -> WebError {
    WebError::AskamaError(err.to_string())
  }
}


impl From<IOError> for WebError {
  fn from(_: IOError) -> WebError {
    WebError::NotFound
  }
}