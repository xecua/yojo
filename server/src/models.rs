use crate::schema::*;

use diesel::{Insertable, Queryable};
use serde_derive::{Deserialize, Serialize};

// DB Models
#[derive(Debug, Insertable, Queryable, Serialize, Deserialize, PartialEq, Eq)]
pub struct Tweet {
    pub id: String,
    pub tweet_id: String,
    pub comment: String,
}

// also used for API Model `Tag`
#[derive(Debug, Insertable, Queryable, Serialize, Deserialize, PartialEq, Eq)]
pub struct Tag {
    pub id: String,
    pub tag: String,
}

#[derive(Debug, Insertable, Queryable, Serialize, Deserialize, Associations, Eq, PartialEq)]
#[belongs_to(Tweet, foreign_key = "tweets_id")]
#[belongs_to(Tag, foreign_key = "tags_id")]
#[table_name = "tweets_to_tags"]
pub struct TweetToTag {
    pub id: String,
    pub tweets_id: String,
    pub tags_id: String,
}

// API Models
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TweetDetail {
    pub id: String,
    pub tweet_id: String,
    pub comment: String,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PostTweet {
    pub tweet_id: String,
    pub comment: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PostTag {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TagDetail {
    pub id: String,
    pub content: String,
    pub tweets: Vec<String>,
}
