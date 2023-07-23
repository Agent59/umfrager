// @generated automatically by Diesel CLI.

diesel::table! {
    gamename (name) {
        name -> Text,
    }
}

diesel::table! {
    usr (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::table! {
    usrpoints (id) {
        id -> Text,
        points -> Integer,
        gamename_id -> Text,
        usr_id -> Text,
    }
}

diesel::joinable!(usrpoints -> gamename (gamename_id));
diesel::joinable!(usrpoints -> usr (usr_id));

diesel::allow_tables_to_appear_in_same_query!(
    gamename,
    usr,
    usrpoints,
);
