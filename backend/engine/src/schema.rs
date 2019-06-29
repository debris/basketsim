table! {
    leagues (id) {
        id -> Integer,
        tier -> Unsigned<Tinyint>,
    }
}

table! {
    players (id) {
        id -> Integer,
        team_id -> Nullable<Integer>,
        first_name -> Text,
        last_name -> Text,
        age -> Unsigned<Tinyint>,
        height -> Unsigned<Tinyint>,
        weight -> Unsigned<Tinyint>,
    }
}

table! {
    teams (id) {
        id -> Integer,
        user_id -> Nullable<Integer>,
        league_id -> Nullable<Integer>,
        name -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        email -> Varchar,
        name -> Text,
        password -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    leagues,
    players,
    teams,
    users,
);
