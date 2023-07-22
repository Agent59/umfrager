use actix_web::{get, post, web, Responder, Result, HttpResponse};
use actix_files::NamedFile;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;

pub fn home_conf(cfg: &mut web::ServiceConfig) {
    cfg.service(home_page);
    cfg.service(survey_data);
    cfg.service(game_names);
}

#[get("/")]
async fn home_page() -> Result<NamedFile> {
    let path = "./static/home.html";
    Ok(NamedFile::open(path)?)
}

#[get("/game_names")]
async fn game_names() -> Result<impl Responder> {
    let game_names_vec: Vec<String> = (vec!["test1", "test2", "test3"])
        .into_iter().map(|s| String::from(s)).collect();
    Ok(web::Json(GameNamesJson { game_names: game_names_vec }))
}

#[derive(Debug, Serialize)]
struct GameNamesJson {
    game_names: Vec<String>,
}


#[post("/survey_data")]
async fn survey_data(json: web::Json<SurveyJson>) -> Result<impl Responder> {
    println!("{:?}", json);

    let writer = File::create(&json.name)?;
    let writer = zstd::Encoder::new(writer, 0)?.auto_finish();
    serde_json::to_writer(writer, &json)?;

    Ok(HttpResponse::Ok())
}

#[derive(Debug, Deserialize, Serialize)]
struct SurveyJson {
    name: String,
    game_names: Vec<GameName>,
}

#[derive(Debug, Deserialize, Serialize)]
struct GameName {
    name: String,
    points: u8,
}
