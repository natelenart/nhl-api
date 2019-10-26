table! {
    game_types (id) {
        id -> Unsigned<Integer>,
        category -> Char,
        description -> Varchar,
        postseason -> Bool,
        value -> Unsigned<Integer>,
    }
}

table! {
    matchups (id) {
        id -> Unsigned<Integer>,
        season_id -> Unsigned<Integer>,
        event_time -> Datetime,
        game_type_id -> Unsigned<Integer>,
        home_team_id -> Unsigned<Integer>,
        away_team_id -> Unsigned<Integer>,
        home_team_score -> Nullable<Unsigned<Integer>>,
        away_team_score -> Nullable<Unsigned<Integer>>,
    }
}

table! {
    roles (id) {
        id -> Unsigned<Integer>,
        role -> Varchar,
    }
}

table! {
    seasons (id) {
        id -> Unsigned<Integer>,
        season -> Char,
    }
}

table! {
    teams (id) {
        id -> Unsigned<Integer>,
        nhl_id -> Unsigned<Integer>,
        abbreviation -> Char,
        location -> Varchar,
        name -> Varchar,
    }
}

table! {
    user_roles (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        role_id -> Unsigned<Integer>,
    }
}

table! {
    user_teams (id) {
        id -> Unsigned<Integer>,
        season_id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        team_id -> Unsigned<Integer>,
    }
}

table! {
    users (id) {
        id -> Unsigned<Integer>,
        username -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        nickname -> Varchar,
        password -> Nullable<Char>,
        salt -> Nullable<Char>,
        email_address -> Nullable<Varchar>,
    }
}

joinable!(matchups -> game_types (game_type_id));
joinable!(matchups -> seasons (season_id));
joinable!(user_roles -> roles (role_id));
joinable!(user_roles -> users (user_id));
joinable!(user_teams -> seasons (season_id));
joinable!(user_teams -> teams (team_id));
joinable!(user_teams -> users (user_id));

allow_tables_to_appear_in_same_query!(
    game_types,
    matchups,
    roles,
    seasons,
    teams,
    user_roles,
    user_teams,
    users,
);
