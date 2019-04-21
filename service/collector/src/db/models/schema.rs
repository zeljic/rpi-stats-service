table! {
    instance (id) {
        id -> Int4,
        uuid -> Varchar,
        name -> Varchar,
        description -> Nullable<Text>,
        enabled -> Bool,
    }
}

table! {
    log (id) {
        id -> Int4,
        instance_id -> Int4,
        log_type_id -> Int4,
        date_time -> Int4,
        value -> Float4,
        enabled -> Bool,
    }
}

table! {
    log_type (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        enabled -> Bool,
    }
}

table! {
    mesh (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        enabled -> Bool,
    }
}

table! {
    mesh_instance (id) {
        id -> Int4,
        mesh_id -> Int4,
        instance_id -> Int4,
        enabled -> Bool,
    }
}

table! {
    user (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        enabled -> Bool,
    }
}

table! {
    user_mesh (id) {
        id -> Int4,
        user_id -> Int4,
        mesh_id -> Int4,
        enabled -> Bool,
    }
}

joinable!(log -> instance (instance_id));
joinable!(log -> log_type (log_type_id));
joinable!(log_type -> user (user_id));
joinable!(mesh_instance -> instance (instance_id));
joinable!(mesh_instance -> mesh (mesh_id));
joinable!(user_mesh -> mesh (mesh_id));
joinable!(user_mesh -> user (user_id));

allow_tables_to_appear_in_same_query!(
    instance,
    log,
    log_type,
    mesh,
    mesh_instance,
    user,
    user_mesh,
);
