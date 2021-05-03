/*
//  Copyright 2021 Catgirl.Moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use crate::error::WebError;

use actix_files::NamedFile;
use actix_web::{get, HttpResponse, web, Scope};
use serde_json::json;
use std::{fs, path::Path};
use serde::Deserialize;
use std::time::UNIX_EPOCH;


// Temporary code before I put the memes into a db
#[derive(Deserialize)]
pub struct Query {
  page: usize
}
pub fn list_files(size: usize, page: usize) -> Result<Vec<String>, std::io::Error> {
  let mut paths: Vec<_> = fs::read_dir("./data")?.map(|r| r.unwrap()).collect();
  paths.sort_by_key(|p| p.path().metadata().expect("meta").modified().expect("modified"));
  paths.reverse();
  let result = paths.into_iter().map(|p| p.path().file_stem().unwrap().to_str().unwrap().to_owned()).collect::<Vec<_>>();
  // LMAO VEC PAGINATION TIME FUCK MEMORY I DONT CARE YEEEEEEEEEEEEEEEEEEe
  Ok(result[std::ops::Range { start: std::cmp::min(size * page, result.len()), end: std::cmp::min(size * (page + 1), result.len()) }].to_vec())
}

#[get("/latest")]
async fn latest_memes(q: web::Query<Query>) -> Result<HttpResponse, WebError> {
  Ok(HttpResponse::Ok().json(json!(list_files(100, q.page)?)))
}

#[get("/{meme:[a-z0-9]+}")]
async fn get_meme(web::Path(meme): web::Path<String>) -> Result<HttpResponse, WebError> {
  Ok(HttpResponse::Ok().json(json!({
    "views": "69+",
    "created": Path::new("./data/").join(meme).with_extension("webm").metadata().expect("meta").modified().expect("modified").duration_since(UNIX_EPOCH).expect("epoc").as_millis() as u64
  })))
}

#[get("/{meme:[a-z0-9]+}/video.webm")]
async fn get_meme_webm(web::Path(meme): web::Path<String>) -> Result<NamedFile, WebError> {
  Ok(NamedFile::open(Path::new("./data/").join(meme).with_extension("webm"))?)
}

#[get("/{meme:[a-z0-9]+}/thumb.webp")]
async fn get_meme_webp(web::Path(meme): web::Path<String>) -> Result<NamedFile, WebError> {
  Ok(NamedFile::open(Path::new("./thumbs/").join(meme).with_extension("webp"))?)
}

#[get("/{meme:[a-z0-9]+}/similar")]
async fn get_meme_similar(web::Path(meme): web::Path<String>, q: web::Query<Query>) -> Result<HttpResponse, WebError> {
  Ok(HttpResponse::Ok().json(json!(list_files(25, q.page)?)))
}

pub fn scope() -> Scope {
  web::scope("/memes")
    .service(latest_memes)
    .service(get_meme)
    .service(get_meme_webm)
    .service(get_meme_webp)
    .service(get_meme_similar)
}