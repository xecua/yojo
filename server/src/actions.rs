#![allow(unused_variables, dead_code)]
use crate::models::*;
use crate::schema;
use crate::DBConnection;
use diesel::prelude::*;

pub fn insert_tweet(
    tweet_id_: &str,
    comment_: &str,
    html_: &str,
    conn: &DBConnection,
) -> QueryResult<Tweet> {
    use schema::tweets::dsl::*;

    let tweet = Tweet {
        id: uuid::Uuid::new_v4().to_string(),
        tweet_id: tweet_id_.to_string(),
        comment: comment_.to_string(),
        html: html_.to_string(),
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

pub fn select_tweet_simples(conn: &DBConnection) -> QueryResult<Vec<TweetSimple>> {
    unimplemented!()
}

pub fn select_tweet_details(conn: &DBConnection) -> QueryResult<Vec<TweetDetail>> {
    unimplemented!()
}

pub fn insert_tag(tag_: &str, conn: &DBConnection) -> QueryResult<Tag> {
    use schema::tags::dsl::*;

    let new_tag = Tag {
        id: uuid::Uuid::new_v4().to_string(),
        tag: tag_.to_string(),
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
    tags.filter(tag.like(format!("{}%", query)))
        .order(id.asc())
        .load(conn)
}

pub fn select_tags(conn: &DBConnection) -> QueryResult<Vec<Tag>> {
    use schema::tags::dsl::*;
    tags.order(id.asc()).load(conn)
}

pub fn select_tag_details(conn: &DBConnection) -> QueryResult<Vec<TagDetail>> {
    unimplemented!()
}

// link inserted records(insert tweets_to_tags)
pub fn link_tweet_and_tags(
    tweet_id: &str,
    tag_ids: Vec<&str>,
    conn: &DBConnection,
) -> QueryResult<Vec<TweetToTag>> {
    use schema::tweets_to_tags::dsl::*;
    let res: Vec<TweetToTag> = tag_ids
        .iter()
        .map(|tag_id| TweetToTag {
            id: uuid::Uuid::new_v4().to_string(),
            tweets_id: tweet_id.to_owned(),
            tags_id: tag_id.to_string(),
        })
        .collect();
    diesel::insert_into(tweets_to_tags)
        .values(&res)
        .execute(conn)?;
    Ok(res)
}

// select tweets_to_tags by tags_id
pub fn get_linked_tweets_to_tag(tag_id: &str, conn: &DBConnection) -> QueryResult<Vec<Tweet>> {
    use schema::{tags::dsl::*, tweets::dsl::*};
    let tag_ = tags.find(tag_id).first::<Tag>(conn)?;
    <TweetToTag as BelongingToDsl<&Tag>>::belonging_to(&tag_)
        .inner_join(tweets)
        .select((schema::tweets::id, tweet_id, comment, html))
        .order(schema::tweets::id.asc())
        .load(conn)
}

// select tweets_to_tags by tweets_id
pub fn get_linked_tags_to_tweet(tweet_id_: &str, conn: &DBConnection) -> QueryResult<Vec<Tag>> {
    use schema::{tags::dsl::*, tweets::dsl::*};
    let tweet_ = tweets.find(tweet_id_).first::<Tweet>(conn)?;
    <TweetToTag as BelongingToDsl<&Tweet>>::belonging_to(&tweet_)
        .inner_join(tags)
        .select((schema::tags::id, tag))
        .order(schema::tags::id.asc())
        .load(conn)
}

// would used with select_tweets/select_tags (to avoid N+1)
pub fn get_tweets_to_tags(conn: &DBConnection) -> QueryResult<Vec<TweetToTag>> {
    use schema::tweets_to_tags::dsl::*;
    tweets_to_tags
        .order(tweets_id.asc())
        .then_order_by(tags_id.asc())
        .load(conn)
}

// TODO: UPDATE, DELETE functions
// note: on update, tweets_to_tags will also be updated
