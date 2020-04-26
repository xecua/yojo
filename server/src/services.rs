//! actix services

use crate::models::*;
use actix_web::{delete, get, patch, post, web::Json, Result as WebResult};

#[get("/tweets")]
pub async fn get_tweets() -> WebResult<Json<Vec<Tweet>>> {
    unimplemented!()
}

#[post("/tweets")]
pub async fn post_tweets() -> WebResult<Json<Tweet>> {
    unimplemented!()
}

#[get("/tweets/{tweet_id}")]
pub async fn get_tweets_id() -> WebResult<Json<Tweet>> {
    unimplemented!()
}

#[delete("/tweets/{tweet_id}")]
pub async fn delete_tweets_id() -> WebResult<Json<Tweet>> {
    unimplemented!()
}

#[patch("/tweets/{tweet_id}")]
pub async fn patch_tweets_id() -> WebResult<Json<Tweet>> {
    unimplemented!()
}

#[get("/tags")]
pub async fn get_tags() -> WebResult<Json<Vec<Tag>>> {
    unimplemented!()
}

#[post("/tags")]
pub async fn post_tags() -> WebResult<Json<Tag>> {
    unimplemented!()
}

#[get("/tags/{tag_id}")]
pub async fn get_tags_id() -> WebResult<Json<Tag>> {
    unimplemented!()
}

#[delete("/tags/{tag_id}")]
pub async fn delete_tags_id() -> WebResult<Json<Tag>> {
    unimplemented!()
}

#[get("/tags/predict")]
pub async fn get_tags_predict() -> WebResult<Json<Vec<Tag>>> {
    unimplemented!()
}
