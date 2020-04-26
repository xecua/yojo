use crate::DBConnection;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};

#[test]
fn insert_tweet() {
    use crate::actions::insert_tweet;

    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = DBConnection::establish(&database_url).unwrap();

    conn.test_transaction::<_, Error, _>(|| {
        let inserted1 = insert_tweet("1", "", &conn)?;
        assert_eq!("1", inserted1.tweet_id);
        assert_eq!("", inserted1.comment);

        let inserted2 = insert_tweet("28049837402", "にほんご", &conn)?;
        assert_eq!("28049837402", inserted2.tweet_id);
        assert_eq!("にほんご", inserted2.comment);

        // ID duplicated
        // POST -> insert_tweet, PATCH -> update_tweet
        // tweet_idが重複したらUniqueViolationになるのでそのままエラーを返す
        // 情報取り出したい...
        let inserted3 = insert_tweet("28049837402", "ほげ", &conn);
        assert!(matches!(
            inserted3,
            Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _))
        ));

        Ok(())
    });
}

#[test]
fn select_tweets() {
    use crate::actions::{insert_tweet, select_tweets};

    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = DBConnection::establish(&database_url).unwrap();

    conn.test_transaction::<_, Error, _>(|| {
        let mut inserted = vec![
            insert_tweet("22", "", &conn)?,
            insert_tweet("1237831", "にほんご", &conn)?,
            insert_tweet("1293467236978", "comment", &conn)?,
        ];
        inserted.sort_by(|left, right| left.id.cmp(&right.id));

        let selected = select_tweets(&conn)?;

        assert_eq!(inserted, selected);
        Ok(())
    });
}

#[test]
fn select_tweet_by_id() {
    use crate::actions::{insert_tweet, select_tweet_by_id};

    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = DBConnection::establish(&database_url).unwrap();

    conn.test_transaction::<_, Error, _>(|| {
        let inserted = insert_tweet("28049837402", "コメント", &conn)?;
        let selected = select_tweet_by_id(&inserted.id, &conn)?;
        assert_eq!(inserted, selected);

        assert_eq!(Err(Error::NotFound), select_tweet_by_id("3", &conn));
        Ok(())
    });
}

#[test]
fn select_tweet_by_tweet_id() {
    use crate::actions::{insert_tweet, select_tweet_by_tweet_id};

    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = DBConnection::establish(&database_url).unwrap();

    conn.test_transaction::<_, Error, _>(|| {
        let inserted = insert_tweet("20309485", "comment", &conn)?;
        let selected = select_tweet_by_tweet_id("20309485", &conn)?;
        assert_eq!(inserted, selected);

        assert_eq!(Err(Error::NotFound), select_tweet_by_tweet_id("2", &conn));
        Ok(())
    });
}

#[test]
fn insert_tag() {
    use crate::actions::insert_tag;

    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = DBConnection::establish(&database_url).unwrap();

    conn.test_transaction::<_, Error, _>(|| {
        let inserted = insert_tag("tag", &conn)?;
        assert_eq!("tag", inserted.tag);

        let inserted = insert_tag("にほんご", &conn)?;
        assert_eq!("にほんご", inserted.tag);

        Ok(())
    });
}

#[test]
fn select_tag_by_content() {
    use crate::actions::{insert_tag, select_tag_by_content};

    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = DBConnection::establish(&database_url).unwrap();

    conn.test_transaction::<_, Error, _>(|| {
        let inserted = insert_tag("foo", &conn)?;
        let selected = select_tag_by_content("foo", &conn)?;
        assert_eq!(inserted, selected);

        assert_eq!(Err(Error::NotFound), select_tag_by_content("bar", &conn));

        Ok(())
    });
}

#[test]
fn select_tag_by_id() {
    use crate::actions::{insert_tag, select_tag_by_id};

    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = DBConnection::establish(&database_url).unwrap();

    conn.test_transaction::<_, Error, _>(|| {
        let inserted = insert_tag("foo", &conn)?;
        let selected = select_tag_by_id(&inserted.id, &conn)?;
        assert_eq!(inserted, selected);

        assert_eq!(Err(Error::NotFound), select_tag_by_id("bar", &conn));

        Ok(())
    });
}

#[test]
fn predict_tag() {
    use crate::actions::{insert_tag, predict_tag};

    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = DBConnection::establish(&database_url).unwrap();

    conn.test_transaction::<_, Error, _>(|| {
        insert_tag("f", &conn)?;
        insert_tag("fizz", &conn)?;
        insert_tag("fo", &conn)?;
        insert_tag("foo", &conn)?;
        insert_tag("foo_bar", &conn)?;
        insert_tag("for", &conn)?;

        let mut predicted = predict_tag("f", &conn)?;
        predicted.sort_by(|left, right| left.tag.cmp(&right.tag));
        assert_eq!(
            vec!["f", "fizz", "fo", "foo", "foo_bar", "for"],
            predicted
                .iter()
                .map(|x| x.tag.to_string())
                .collect::<Vec<String>>()
        );

        let mut predicted = predict_tag("fo", &conn)?;
        predicted.sort_by(|left, right| left.tag.cmp(&right.tag));
        assert_eq!(
            vec!["fo", "foo", "foo_bar", "for"],
            predicted
                .iter()
                .map(|x| x.tag.to_string())
                .collect::<Vec<String>>()
        );

        let predicted = predict_tag("for", &conn)?;
        assert_eq!("for", predicted[0].tag);

        let mut predicted = predict_tag("foo", &conn)?;
        predicted.sort_by(|left, right| left.tag.cmp(&right.tag));
        assert_eq!(
            vec!["foo", "foo_bar"],
            predicted
                .iter()
                .map(|x| x.tag.to_string())
                .collect::<Vec<String>>()
        );

        let predicted = predict_tag("foo_", &conn)?;
        assert_eq!(
            vec!["foo_bar"],
            predicted
                .iter()
                .map(|x| x.tag.to_string())
                .collect::<Vec<String>>()
        );

        Ok(())
    });
}

#[test]
fn select_tags() {
    use crate::actions::{insert_tag, select_tags};

    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = DBConnection::establish(&database_url).unwrap();

    conn.test_transaction::<_, Error, _>(|| {
        let mut inserted = vec![
            insert_tag("tag1", &conn)?,
            insert_tag("タグ", &conn)?,
            insert_tag("", &conn)?,
        ];
        inserted.sort_by(|left, right| left.id.cmp(&right.id));
        let selected = select_tags(&conn)?;
        assert_eq!(inserted, selected);
        Ok(())
    });
}
