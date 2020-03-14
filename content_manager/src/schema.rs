
table! {
	content (content_id) {
		content_id -> Integer,
		site_id -> Integer,
		inner_path -> Text,
		size -> Integer,
		size_files -> Integer,
		size_files_optional -> Integer,
		modified -> Integer,
	}
}

table! {
	site (site_id) {
		site_id -> Integer,
		address -> Text,
	}
}

table! {
	file_optional (file_id) {
		file_id -> Integer,
		site_id -> Integer,
		inner_path -> Text,
		hash_id -> Integer,
		size -> Integer,
		peer -> Integer,
		uploaded -> Integer,
		is_downloaded -> Integer,
		is_pinned -> Integer,
		time_added -> Integer,
		time_downloaded -> Integer,
		time_accessed -> Integer,
	}
}

table! {
	json (json_id) {
		json_id -> Integer,
		site -> Text,
		directory -> Text,
		file_name -> Text,
	}
}