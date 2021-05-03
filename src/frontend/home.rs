/*
//  Copyright 2021 Catgirl.Moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use crate::error::WebError;

use actix_web::{get, HttpResponse};
use actix_http::http::header::ContentType;
use askama::Template;


#[derive(Template)]
#[template(path = "pages/home.html")]
struct HomePage{}

#[get("/")]
async fn render() -> Result<HttpResponse, WebError> {
  Ok(HttpResponse::Ok().set(ContentType(mime::TEXT_HTML)).body(HomePage{}.render()?))
}