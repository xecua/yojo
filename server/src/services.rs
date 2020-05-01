//! actix services

use crate::actions::*;
use crate::models::*;
use crate::Pool;
use actix_web::{delete, get, patch, post, web, HttpResponse, Result as WebResult};
#[allow(unused_imports)]
use log::{debug, error, info, warn};

#[get("/tweets")] // returns OK(Vec<TweetDetail>)
pub async fn get_tweets(pool: web::Data<Pool>) -> WebResult<HttpResponse> {
    let conn = pool.get().expect("Failed to establish connection");
    let res = web::block(move || select_tweets(&conn))
        .await
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().message_body(e);
        })?;
    Ok(HttpResponse::Ok().json(res))
}

// URLをPOST→oembed APIでHTMLを取得→DBに保存
#[post("/tweets")] // returns NoContent
pub async fn post_tweets(
    data: web::Json<PostTweet>,
    pool: web::Data<Pool>,
) -> WebResult<HttpResponse> {
    let conn = pool.get().expect("Failed to establish connection");
    let res = web::block(move || {
        let tweet = insert_tweet(&data.tweet_id, &data.comment, &conn)?;
        if let Err(e) = link_tweet_and_tags(
            &tweet.id,
            data.tags.iter().map(AsRef::as_ref).collect(),
            &conn,
        ) {
            return Err(e);
        }
        Ok(tweet)
    })
    .await
    .map_err(|e| {
        error!("{}", e);
        HttpResponse::InternalServerError().message_body(e);
    })?;
    Ok(HttpResponse::Ok().json(res))
}

#[get("/tweets/{tweet_id}")] // returns OK(TweetDetail)
pub async fn get_tweets_id(
    tweet_id: web::Path<String>,
    pool: web::Data<Pool>,
) -> WebResult<HttpResponse> {
    let conn = pool.get().expect("Failed to establish connection");
    let res = web::block(move || select_tweet_by_tweet_id(&tweet_id, &conn))
        .await
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().message_body(e);
        })?;
    Ok(HttpResponse::Ok().json(res))
}

#[delete("/tweets/{tweet_id}")] // returns NoContent
pub async fn delete_tweets_id() -> WebResult<HttpResponse> {
    unimplemented!()
}

#[patch("/tweets/{tweet_id}")] // returns NoContent
pub async fn patch_tweets_id() -> WebResult<HttpResponse> {
    unimplemented!()
}

#[get("/tags")] // returns OK(Vec<TagDetail>)
pub async fn get_tags(pool: web::Data<Pool>) -> WebResult<HttpResponse> {
    let conn = pool.get().expect("Failed to establish connection");
    let res = web::block(move || select_tags(&conn)).await.map_err(|e| {
        error!("{}", e);
        HttpResponse::InternalServerError().message_body(e);
    })?;
    Ok(HttpResponse::Ok().json(res))
}

#[post("/tags")]
pub async fn post_tags(data: web::Json<PostTag>, pool: web::Data<Pool>) -> WebResult<HttpResponse> {
    let conn = pool.get().expect("Failed to establish connection");
    let res = web::block(move || insert_tag(&data.content, &conn))
        .await
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().message_body(e);
        })?;
    Ok(HttpResponse::Ok().json(res))
}

#[get("/tags/{tag_id}")]
pub async fn get_tags_id(
    tag_id: web::Path<String>,
    pool: web::Data<Pool>,
) -> WebResult<HttpResponse> {
    let conn = pool.get().expect("Failed to establish connection");
    let res = web::block(move || select_tag_by_id(&tag_id, &conn))
        .await
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().message_body(e);
        })?;
    Ok(HttpResponse::Ok().json(res))
}

#[delete("/tags/{tag_id}")]
pub async fn delete_tags_id() -> WebResult<HttpResponse> {
    unimplemented!()
}

#[get("/tags/predict")]
pub async fn get_tags_predict(
    query: web::Query<String>,
    pool: web::Data<Pool>,
) -> WebResult<HttpResponse> {
    let conn = pool.get().expect("Failed to establish connection");
    let res = web::block(move || predict_tag(&query, &conn))
        .await
        .map_err(|e| {
            error!("{}", e);
            HttpResponse::InternalServerError().message_body(e);
        })?;
    Ok(HttpResponse::Ok().json(res))
}
