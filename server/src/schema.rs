table! {
    tags (id) {
        id -> Varchar,
        tag -> Varchar,
    }
}

table! {
    tweets (id) {
        id -> Varchar,
        tweet_id -> Varchar,
        comment -> Varchar,
        html -> Varchar,
    }
}

table! {
    tweets_to_tags (id) {
        id -> Varchar,
        tweets_id -> Varchar,
        tags_id -> Varchar,
    }
}

joinable!(tweets_to_tags -> tags (tags_id));
joinable!(tweets_to_tags -> tweets (tweets_id));

allow_tables_to_appear_in_same_query!(
    tags,
    tweets,
    tweets_to_tags,
);
