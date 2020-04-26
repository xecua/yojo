#![allow(unused_variables, dead_code)]
use crate::models::*;
use crate::schema;
use crate::DBConnection;
use diesel::prelude::*;


pub fn insert_tweet(tweet_id_: &str, comment_: &str, conn: &DBConnection) -> QueryResult<Tweet> {
    use schema::tweets::dsl::*;

    let tweet = Tweet {
        id: uuid::Uuid::new_v4().to_string(),
        tweet_id: tweet_id_.to_string(),
        comment: comment_.to_string(),
    };

    diesel::insert_into(tweets).values(&tweet).execute(conn)?;

    Ok(tweet)
}

pub fn select_tweet_by_id(id: &str, conn: &DBConnection) -> QueryResult<Tweet> {
    unimplemented!()
}

pub fn select_tweet_by_tweet_id(tweet_id: &str, conn: &DBConnection) -> QueryResult<Tweet> {
    unimplemented!()
}

pub fn select_tweets(conn: &DBConnection) -> QueryResult<Vec<Tweet>> {
    unimplemented!()
}

pub fn insert_tag(tag_: &str, conn: &DBConnection) -> QueryResult<Tag> {
    unimplemented!()
}

pub fn select_tag_by_id(id: &str, conn: &DBConnection) -> QueryResult<Tag> {
    unimplemented!()
}

pub fn select_tag_by_content(content: &str, conn: &DBConnection) -> QueryResult<Tag> {
    unimplemented!()
}

pub fn predict_tag(query: &str, conn: &DBConnection) -> QueryResult<Vec<Tag>> {
    unimplemented!()
}

pub fn select_tags(conn: &DBConnection) -> QueryResult<Vec<Tag>> {
    unimplemented!()
}
