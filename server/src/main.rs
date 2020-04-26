use actix_web::{App, HttpServer};
use diesel::r2d2;
use server::services::*;
use server::DBConnection;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set.");
    let manager = r2d2::ConnectionManager::<DBConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build_unchecked(manager);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(get_tweets)
            .service(post_tweets)
            .service(get_tweets_id)
            .service(get_tags)
            .service(post_tags)
            .service(get_tags_id)
            .service(get_tags_predict)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
