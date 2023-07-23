use diesel::{SqliteConnection, r2d2};

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

pub mod models;
pub mod actions;
pub mod schema;

pub mod home;
pub mod results;
