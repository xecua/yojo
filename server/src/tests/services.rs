// integrated tests?(because of Databases)

// use crate::actions::*;
// use crate::DBConnection;
// use actix_web::test;
// use actix_web::{web, App};
// use diesel::prelude::*;
// use diesel::r2d2;

// #[derive(Debug)]
// struct CustomizedConnectionConfig;
// impl r2d2::CustomizeConnection<DBConnection, r2d2::Error> for CustomizedConnectionConfig {
//     fn on_acquire(&self, conn: &mut DBConnection) -> Result<(), r2d2::Error> {
//         conn.begin_test_transaction().unwrap();
//         Ok(())
//     }
// }

// #[actix_rt::test]
// async fn get_tweets() {
//     use crate::services::get_tweets;

//     dotenv::dotenv().ok();
//     let database_url = std::env::var("DATABASE_URL").expect("set DATABASE_URL");
//     let manager = r2d2::ConnectionManager::<DBConnection>::new(database_url);
//     let pool = r2d2::Pool::builder()
//         .connection_customizer(Box::new(CustomizedConnectionConfig))
//              ↑ data rollbacked on each handler...
//         .build(manager)
//         .unwrap();
//     let mut app = test::init_service(App::new().data(pool.clone()).service(get_tweets)).await;
// }
