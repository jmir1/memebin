/*
//  Copyright 2021 Catgirl.Moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use crate::error::WebError;

use actix_web::{get, HttpResponse, web};
use actix_http::http::header::ContentType;
use askama::Template;


#[derive(Template)]
#[template(path = "pages/meme.html")]
struct MemePage{
  meme: String
}

#[get("/{meme:[a-z0-9]+}")]
async fn render(web::Path(meme): web::Path<String>) -> Result<HttpResponse, WebError> {
  Ok(HttpResponse::Ok().set(ContentType(mime::TEXT_HTML)).body(MemePage{meme}.render()?))
}