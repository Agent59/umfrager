use actix_web::{get, web, Responder, Result, error};
use actix_files::NamedFile;
use serde::Serialize;

use crate::{DbPool, actions};

pub fn results_conf(cfg: &mut web::ServiceConfig) {
    cfg.service(results_page);
    cfg.service(get_results_data);
}

#[get("/results")]
async fn results_page() -> Result<NamedFile> {
    let path = "./static/results.html";
    Ok(NamedFile::open(path)?)
}

#[get("/get_results_data")]
async fn get_results_data(pool: web::Data<DbPool>) -> Result<impl Responder> {
    let survey_result = web::block(move || {
        let mut conn = pool.get()?;
        actions::get_survey_result(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(web::Json(survey_result))
}

#[derive(Debug, Serialize)]
pub struct SurveyResultJson {
    pub usrpoints: Vec<UsrPointsJson>,
}

#[derive(Debug, Serialize)]
pub struct UsrPointsJson {
    pub points: i32,
    pub gamename: String,
    pub usr_name: String,
    pub usr_id: String,
}
