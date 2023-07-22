use actix_web::{App, HttpServer, Result, get};
use actix_files::{Files, NamedFile};

use umfrager::home::home_conf;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(home_conf)
            .service(default_style)
            .service(Files::new("/images", "static/images").show_files_listing())
    })
    .bind(("0.0.0.0", 1515))?
    .run()
    .await
}

#[get("/default.css")]
async fn default_style() -> Result<NamedFile> {
    let path = "./static/default.css";
    Ok(NamedFile::open(path)?)
}
