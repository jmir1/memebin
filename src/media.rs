/*
//  Copyright 2021 Catgirl.Moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use crate::error::WebError;

use actix_files::NamedFile;
use actix_web::{get, web, Scope};
use std::path::Path;


#[get("/{meme:[a-z0-9]+}/full.webm")]
async fn webm_full(web::Path(meme): web::Path<String>) -> Result<NamedFile, WebError> {
  Ok(NamedFile::open(Path::new("./data/").join(meme).with_extension("webm"))?)
}

#[get("/{meme:[a-z0-9]+}/thumb.webp")]
async fn webp_thumbnail(web::Path(meme): web::Path<String>) -> Result<NamedFile, WebError> {
  Ok(NamedFile::open(Path::new("./thumbs/").join(meme).with_extension("webp"))?)
}

pub fn scope() -> Scope {
  web::scope("/m")
    .service(webm_full)
    .service(webp_thumbnail)
}