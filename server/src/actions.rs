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

pub fn select_tweet_by_id(id_: &str, conn: &DBConnection) -> QueryResult<Tweet> {
    use schema::tweets::dsl::*;
    tweets.filter(id.eq(id_)).first(conn)
}

pub fn select_tweet_by_tweet_id(tweet_id_: &str, conn: &DBConnection) -> QueryResult<Tweet> {
    use schema::tweets::dsl::*;
    tweets.filter(tweet_id.eq(tweet_id_)).first(conn)
}

pub fn select_tweets(conn: &DBConnection) -> QueryResult<Vec<Tweet>> {
    use schema::tweets::dsl::*;
    tweets.order(id.asc()).load(conn)
}

pub fn insert_tag(tag_: &str, conn: &DBConnection) -> QueryResult<Tag> {
    use schema::tags::dsl::*;

    let new_tag = Tag {
        id: uuid::Uuid::new_v4().to_string(),
        tag: tag_.to_string()
    };

    diesel::insert_into(tags).values(&new_tag).execute(conn)?;

    Ok(new_tag)
}

pub fn select_tag_by_id(id_: &str, conn: &DBConnection) -> QueryResult<Tag> {
    use schema::tags::dsl::*;
    tags.filter(id.eq(id_)).first(conn)
}

pub fn select_tag_by_content(content: &str, conn: &DBConnection) -> QueryResult<Tag> {
    use schema::tags::dsl::*;
    tags.filter(tag.eq(content)).first(conn)
}

pub fn predict_tag(query: &str, conn: &DBConnection) -> QueryResult<Vec<Tag>> {
    use schema::tags::dsl::*;
    tags.filter(tag.like(format!("{}%", query))).order(id.asc()).load(conn)
}

pub fn select_tags(conn: &DBConnection) -> QueryResult<Vec<Tag>> {
    use schema::tags::dsl::*;
    tags.order(id.asc()).load(conn)
}

// link inserted records(insert tweets_to_tags)
pub fn link_tweet_and_tags(tweet_id: &str, tag_ids: Vec<&str>, conn: &DBConnection) -> QueryResult<Vec<TweetToTag>> {
    unimplemented!()
}

// select tweets_to_tags by tags_id
pub fn get_linked_tweets_to_tag(tag_id: &str, conn: &DBConnection) -> QueryResult<Vec<Tweet>> {
    unimplemented!()
}

// select tweets_to_tags by tweets_id
pub fn get_linked_tags_to_tweet(tweet_id: &str, conn: &DBConnection) -> QueryResult<Vec<Tag>> {
    unimplemented!()
}

// would used with select_tweets/select_tags (to avoid N+1)
pub fn get_tweets_to_tags(conn: &DBConnection) -> QueryResult<Vec<TweetToTag>> {
    unimplemented!()
}
