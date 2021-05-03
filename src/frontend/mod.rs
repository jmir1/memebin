/*
//  Copyright 2021 Catgirl.Moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use actix_web::{web, Scope};

mod home;
mod memes;

pub fn scope() -> Scope {
  web::scope("")
    .service(home::render)
    .service(memes::render)
}