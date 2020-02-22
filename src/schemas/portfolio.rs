table! {
    master (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

table! {
    projects (id) {
        id -> Int4,
        app_type -> Varchar,
        deployed_url -> Varchar,
        description -> Varchar,
        game_file -> Varchar,
        git_url -> Varchar,
        icon_file -> Varchar,
        style_file -> Varchar,
        title -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    master,
    projects,
);
