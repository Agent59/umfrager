use actix_web::{get, post, web, Responder, Result, HttpResponse, error};
use actix_files::NamedFile;
use serde::{Deserialize, Serialize};

use crate::{DbPool, actions, models::NewUsrPoints};

pub fn home_conf(cfg: &mut web::ServiceConfig) {
    cfg.service(home_page);
    cfg.service(thanks_page);
    cfg.service(survey_data);
    cfg.service(game_names);
}

#[get("/")]
async fn home_page() -> Result<NamedFile> {
    let path = "./static/home.html";
    Ok(NamedFile::open(path)?)
}

#[get("/thanks")]
async fn thanks_page() -> Result<NamedFile> {
    let path = "./static/thanks.html";
    Ok(NamedFile::open(path)?)
}

#[get("/game_names")]
async fn game_names(pool: web::Data<DbPool>) -> Result<impl Responder> {
    let game_names_option = web::block(move || {
        let mut conn = pool.get()?;
        actions::get_all_game_names(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    match game_names_option {
        Some(game_names_vec) => {
            let game_names_string_vec = game_names_vec.iter().map(|gn| gn.name.clone()).collect();
            Ok(web::Json(GameNamesJson { game_names: game_names_string_vec}))
        },
        None => Err(error::ErrorInternalServerError("game names table empty")),
    }
}

#[derive(Debug, Serialize)]
struct GameNamesJson {
    game_names: Vec<String>,
}


#[post("/survey_data")]
async fn survey_data(pool: web::Data<DbPool>, json: web::Json<SurveyJson>) -> Result<impl Responder> {
    web::block(move || {
        let mut conn = pool.get()?;
        match actions::insert_new_user(&mut conn, &json.name) {
            Ok(new_user) => {
                let new_usrpoints_vec = json.game_names.iter()
                    .map(|gn|
                        NewUsrPoints {
                            points: gn.points,
                            gamename_id: gn.name.clone(),
                            usr_id: new_user.id.clone(),
                        })
                    .collect();
                actions::multi_insert_new_usrpoints(&mut conn, new_usrpoints_vec)
            },
            Err(e) => Err(e),
        }
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok())
}

#[derive(Debug, Deserialize, Serialize)]
struct SurveyJson {
    name: String,
    game_names: Vec<GameNameJson>,
}

#[derive(Debug, Deserialize, Serialize)]
struct GameNameJson {
    name: String,
    points: i32,
}
