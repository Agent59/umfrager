use actix_web::{App, HttpServer, Result, get, web, middleware};
use actix_files::{Files, NamedFile};
use diesel::{SqliteConnection, r2d2};

use umfrager::home::home_conf;
use umfrager::results::results_conf;
use umfrager::DbPool;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let pool = initialize_db_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .configure(home_conf)
            .configure(results_conf)
            .service(default_style)
            .service(Files::new("/images", "static/images").show_files_listing())
    })
    .bind(("0.0.0.0", 1515))?
    .run()
    .await
}

fn initialize_db_pool() -> DbPool {
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(conn_spec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file")
}

#[get("/default.css")]
async fn default_style() -> Result<NamedFile> {
    let path = "./static/default.css";
    Ok(NamedFile::open(path)?)
}
