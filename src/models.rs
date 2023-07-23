use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};

use crate::schema::{gamename, usr, usrpoints};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = gamename)]
pub struct GameName {
    pub name: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = usr)]
pub struct User {
    pub id: String,
    pub name: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(belongs_to(GameName))]
#[diesel(belongs_to(User))]
#[diesel(table_name = usrpoints)]
pub struct UsrPoints {
    pub id: String,
    pub points: i32,
    pub gamename_id: String,
    pub usr_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUsrPoints {
    pub points: i32,
    pub gamename_id: String,
    pub usr_id: String,
}
