//! actix services

use crate::actions::*;
use crate::models::*;
use crate::Pool;
use actix_web::{delete, get, patch, post, web, HttpResponse, Result as WebResult};
#[allow(unused_imports)]
use log::{debug, error, info, warn};

lazy_static::lazy_static! {
    static ref TWEET_URL: regex::Regex = regex::Regex::new(r"https?://twitter\.com/trapyojo/status/(\d+).*").unwrap();
}

#[get("/tweets")] // returns OK(Vec<TweetDetail>)
pub async fn get_tweets(pool: web::Data<Pool>) -> WebResult<HttpResponse> {
    let conn = pool.get().expect("Failed to establish connection");
    let res = web::block(move || select_tweet_details(&conn)).await?;
    Ok(HttpResponse::Ok().json(res))
}

// URLをPOST→oembed APIでHTMLを取得→DBに保存
#[post("/tweets")] // returns NoContent
pub async fn post_tweets(
    data: web::Json<PostTweet>,
    pool: web::Data<Pool>,
) -> WebResult<HttpResponse> {
    use std::collections::HashMap;

    let mut query: HashMap<&str, &str> = HashMap::new();
    query.insert("url", &data.link);
    query.insert("lang", "ja");
    query.insert("omit_script", "t");

    let html = actix_web::client::Client::default()
        .get("https://publish.twitter.com/oembed")
        .query(&query)
        .unwrap()
        .send()
        .await?
        .json::<EmbedAPIResponse>()
        .await?
        .html;

    let conn = pool.get().expect("Failed to establish connection");
    web::block::<_, _, diesel::result::Error>(move || {
        let id = &(*TWEET_URL).captures(&data.link).unwrap()[1];
        let tweet = insert_tweet(id, &data.comment, &html, &conn)?;
        link_tweet_and_tags(
            &tweet.id,
            data.tags.iter().map(AsRef::as_ref).collect(),
            &conn,
        )?;
        Ok(())
    })
    .await?;
    Ok(HttpResponse::NoContent().finish())
}

#[get("/tweets/{tweet_id}")] // returns OK(TweetDetail)
pub async fn get_tweets_id(
    tweet_id: web::Path<String>,
    pool: web::Data<Pool>,
) -> WebResult<HttpResponse> {
    let conn = pool.get().expect("Failed to establish connection");
    let res = web::block(move || select_tweet_by_id(&tweet_id, &conn)).await?;
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
    let res = web::block(move || select_tags(&conn)).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[post("/tags")]
pub async fn post_tags(data: web::Json<PostTag>, pool: web::Data<Pool>) -> WebResult<HttpResponse> {
    let conn = pool.get().expect("Failed to establish connection");
    let res = web::block(move || insert_tag(&data.content, &conn)).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[get("/tags/{tag_id}")]
pub async fn get_tags_id(
    tag_id: web::Path<String>,
    pool: web::Data<Pool>,
) -> WebResult<HttpResponse> {
    let conn = pool.get().expect("Failed to establish connection");
    let res = web::block(move || select_tag_by_id(&tag_id, &conn)).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[delete("/tags/{tag_id}")]
pub async fn delete_tags_id() -> WebResult<HttpResponse> {
    unimplemented!()
}

#[get("/tags/predict")]
pub async fn get_tags_predict(
    web::Query(query): web::Query<SimpleQuery>,
    pool: web::Data<Pool>,
) -> WebResult<HttpResponse> {
    let conn = pool.get().expect("Failed to establish connection");
    let res = web::block(move || predict_tag(&query.q, &conn)).await?;
    Ok(HttpResponse::Ok().json(res))
}
