#[macro_use]
extern crate diesel;

pub mod actions;
pub mod services;
pub mod models;
mod schema;

#[cfg(test)]
mod tests;
#[cfg(test)]
#[macro_use]
extern crate diesel_migrations;

type DBConnection = diesel::mysql::MysqlConnection;
