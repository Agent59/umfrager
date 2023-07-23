use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;

use crate::models;
use crate::results::{SurveyResultJson, UsrPointsJson};

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn insert_new_game(
    conn: &mut SqliteConnection,
    nm: &str,
) -> Result<models::GameName, DbError> {
    use crate::schema::gamename::dsl::*;

    let new_gamename = models::GameName {
        name: nm.to_owned(),
    };

    diesel::insert_into(gamename).values(&new_gamename).execute(conn)?;
    Ok(new_gamename)
}

pub fn insert_new_games(
    conn: &mut SqliteConnection,
    names: Vec<String>,
) -> Result<Vec<models::GameName>, DbError> {
    use crate::schema::gamename::dsl::*;

    let new_gamenames: Vec<models::GameName> = names.iter().map(|nm| models::GameName { name: nm.to_owned() }).collect();

    diesel::insert_into(gamename).values(&new_gamenames).execute(conn)?;
    Ok(new_gamenames)
}

pub fn get_all_game_names(conn: &mut SqliteConnection)
-> Result<Option<Vec<models::GameName>>, DbError> {
    use crate::schema::gamename::dsl::*;

    let gamenames = gamename.filter(name.is_not_null())
        .get_results::<models::GameName>(conn)
        .optional()?;
    Ok(gamenames)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_user(
    conn: &mut SqliteConnection,
    nm: &str, // prevent collision with `name` column imported inside the function
) -> Result<models::User, DbError> {
    // It is common when using Diesel with Actix Web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::usr::dsl::*;

    let new_user = models::User {
        id: Uuid::new_v4().to_string(),
        name: nm.to_owned(),
    };

    diesel::insert_into(usr).values(&new_user).execute(conn)?;
    Ok(new_user)
}

pub fn find_user_by_id(
    conn: &mut SqliteConnection,
    id_string: String,
) -> Result<Option<models::User>, DbError> {
    use crate::schema::usr::dsl::*;

    let user = usr
        .find(id_string)
        .first(conn)
        .optional()?;
    Ok(user)
}

pub fn insert_new_usrpoints(
    conn: &mut SqliteConnection,
    pts: i32,
    gnm_id: String, // gamename id is gamename.name
    u_id: String,
) -> Result<models::UsrPoints, DbError> {
    use crate::schema::usrpoints::dsl::*;

    let new_usrpoints = models::UsrPoints {
        id: Uuid::new_v4().to_string(),
        points: pts,
        gamename_id: gnm_id.to_owned(),
        usr_id: u_id.to_owned(),
    };

    diesel::insert_into(usrpoints).values(&new_usrpoints).execute(conn)?;
    Ok(new_usrpoints)
}

pub fn multi_insert_new_usrpoints(
    conn: &mut SqliteConnection,
    new_usrpoints_vec: Vec<models::NewUsrPoints>,
) -> Result<Vec<models::UsrPoints>, DbError> {
    let mut usrpoints_vec: Vec<models::UsrPoints> = Vec::new();

    for nup in new_usrpoints_vec {
        let newly_inserted = insert_new_usrpoints(conn, nup.points, nup.gamename_id, nup.usr_id)?;
        usrpoints_vec.push(newly_inserted);
    }
    
    Ok(usrpoints_vec)
}

pub fn get_all_usrpoints(conn: &mut SqliteConnection)
-> Result<Option<Vec<models::UsrPoints>>, DbError> {
    use crate::schema::usrpoints::dsl::*;

    let usrpoints_vec_option = usrpoints.filter(id.is_not_null())
        .get_results::<models::UsrPoints>(conn)
        .optional()?;
    Ok(usrpoints_vec_option)
}


pub fn get_survey_result(
    conn: &mut SqliteConnection,

) -> Result<SurveyResultJson, DbError> {
    use crate::schema::usrpoints::dsl::*;

    let usrpoints_vec_option = usrpoints.filter(id.is_not_null())
        .get_results::<models::UsrPoints>(conn)
        .optional()?;

    match usrpoints_vec_option {
        Some(up_vec) => {
            let mut up_vec_json: Vec<UsrPointsJson> = Vec::new();
            for up in up_vec.iter() {
                let usrn = find_user_by_id(conn, up.usr_id.clone())?.ok_or(Error::NotFound)?.name;
                up_vec_json.push(UsrPointsJson {
                    points: up.points,
                    gamename: up.gamename_id.clone(),
                    usr_name: usrn,
                    usr_id: up.usr_id.clone(),
                });
            }
            Ok(SurveyResultJson { usrpoints: up_vec_json })
        },
        None => Err(Box::new(Error::NotFound)),
    }
}
