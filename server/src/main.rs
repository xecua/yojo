use actix_web::{middleware::Logger, App, HttpServer};
use diesel::r2d2;
use listenfd::ListenFd;
use server::services::*;
use server::DBConnection;
use actix_cors::Cors;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set.");
    let manager = r2d2::ConnectionManager::<DBConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build_unchecked(manager);

    std::env::set_var("RUST_LOG", "DEBUG");
    env_logger::init();

    let mut listenfd = ListenFd::from_env();
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::new().send_wildcard().finish())
            .data(pool.clone())
            .service(get_tweets)
            .service(post_tweets)
            .service(get_tweets_id)
            .service(get_tags)
            .service(post_tags)
            .service(get_tags_predict)
            .service(get_tags_id)
    });

    if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    }
    .run()
    .await
}
