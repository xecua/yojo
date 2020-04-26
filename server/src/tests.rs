mod actions;

mod example_check {
    // https://docs.diesel.rs/diesel/connection/trait.Connection.html#example-1
    diesel::table! {
        users (name) {
            name -> Varchar,
        }
    }

    #[test]
    fn does_example_runs() {
        use self::users::dsl::*;
        use diesel::prelude::*;
        use diesel::result::Error;

        embed_migrations!("tests/migrations");
        let conn = diesel::sqlite::SqliteConnection::establish(":memory:").unwrap();
        embedded_migrations::run(&conn).unwrap();

        conn.test_transaction::<_, Error, _>(|| {
            diesel::insert_into(users)
                .values(name.eq("Ruby"))
                .execute(&conn)?;

            let all_names = users.select(name).load::<String>(&conn)?;
            assert_eq!(vec!["Sean", "Tess", "Ruby"], all_names);

            Ok(())
        });

        // Even though we returned `Ok`, the transaction wasn't committed.
        let all_names = users.select(name).load::<String>(&conn).unwrap();
        assert_eq!(vec!["Sean", "Tess"], all_names);
    }
}
