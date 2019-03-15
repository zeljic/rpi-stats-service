table! {
	instance (id) {
		id -> Integer,
		uuid -> Text,
		name -> Text,
		description -> Nullable<Text>,
		enabled -> Bool,
	}
}

table! {
	log (id) {
		id -> Integer,
		instance_id -> Integer,
		log_type_id -> Integer,
		date_time -> Integer,
		value -> Float,
		enabled -> Bool,
	}
}

table! {
	log_type (id) {
		id -> Integer,
		user_id -> Integer,
		name -> Text,
		description -> Nullable<Text>,
		enabled -> Bool,
	}
}

table! {
	mesh (id) {
		id -> Integer,
		name -> Text,
		description -> Nullable<Text>,
		enabled -> Bool,
	}
}

table! {
	mesh_instance (id) {
		id -> Integer,
		mesh_id -> Integer,
		instance_id -> Integer,
		enabled -> Bool,
	}
}

table! {
	user (id) {
		id -> Integer,
		name -> Nullable<Text>,
		email -> Nullable<Text>,
		password -> Nullable<Text>,
		enabled -> Bool,
	}
}

table! {
	user_mesh (id) {
		id -> Integer,
		user_id -> Integer,
		mesh_id -> Integer,
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
