#[macro_use]
extern crate diesel;

mod actions;
pub mod models;
mod schema;
pub mod services;

#[cfg(test)]
mod tests;
#[cfg(test)]
#[macro_use]
extern crate diesel_migrations;

pub type DBConnection = diesel::mysql::MysqlConnection;
type Pool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<DBConnection>>;
