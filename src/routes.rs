use actix_web::{get, post, web, HttpResponse, Responder};
use diesel::prelude::*;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

use crate::{db, models::NewUrl};
use crate::models::Url;
use crate::schema::url::dsl::*;

use sha2::{Sha256, Digest};

const BASE62: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn base62_encode(mut num: u128) -> String {
    if num == 0 {
        return "0".to_string();
    }

    let mut encoded = Vec::new();
    while num > 0 {
        let rem = (num % 62) as usize;
        encoded.push(BASE62[rem]);
        num /= 62;
    }

    encoded.reverse();
    String::from_utf8(encoded).unwrap()
}

pub fn generate_short_code(_url: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(_url.as_bytes());
    let result = hasher.finalize();

    let num = u128::from_be_bytes(result[0..16].try_into().unwrap());

    let mut short = base62_encode(num);

    short.truncate(8);
    short
}

fn normalize_url(_url: &str) -> String {
    if _url.starts_with("http://") || _url.starts_with("https://") {
        _url.to_string()
    } else {
        format!("https://{}", _url) 
}
}

#[get("/list")]
pub async fn list_urls(pool: web::Data<db::DbPool>) -> impl Responder {
    let mut conn = pool.get().unwrap();
    let result = url.load::<Url>(&mut conn).expect("error loading urls");
    HttpResponse::Ok().json(result)
}
#[derive(Deserialize, Serialize)]
struct Url_coming{
    url:String
}

#[post("/create")]
pub async fn create_link(pool: web::Data<db::DbPool>, url_json : web::Json<Url_coming>)  -> impl Responder{
    let mut conn = pool.get().unwrap();
    let shorten = NewUrl{
        original_url: &url_json.url,
        shorten_url : &generate_short_code(&normalize_url(&url_json.url))
    };
    diesel::insert_into(url)
            .values(&shorten)
            .execute(&mut conn)
            .unwrap();
    
    HttpResponse::Ok().json(shorten)
}   

#[get("/r/{url}")]
pub async  fn visit(uri: web::Path<String>, pool: web::Data<db::DbPool>, ) -> impl Responder {
    let mut conn = pool.get().unwrap();
    let short_url = uri.to_string();
    let matching_url = url.filter(shorten_url.eq(short_url)).first::<Url>(&mut conn);

    match &matching_url {
        Ok(other_url) => HttpResponse::Found().append_header(("Location", other_url.original_url.clone())).finish(),
        Err(diesel::result::Error::NotFound)  => HttpResponse::NotFound().body("not url found"),
        _ => HttpResponse::InternalServerError().body("internal error")
    }
}