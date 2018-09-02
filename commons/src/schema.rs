table! {
	instance (id) {
		id -> Nullable<Integer>,
		uuid -> Text,
		name -> Text,
		enabled -> Bool,
	}
}

table! {
	log (id) {
		id -> Nullable<Integer>,
		instance_id -> Integer,
		log_type_id -> Integer,
		date_time -> Integer,
		value -> Float,
	}
}

table! {
	log_type (id) {
		id -> Nullable<Integer>,
		name -> Text,
		enabled -> Bool,
	}
}

joinable!(log -> instance (instance_id));
joinable!(log -> log_type (log_type_id));

allow_tables_to_appear_in_same_query!(instance, log, log_type,);
