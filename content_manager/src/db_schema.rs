use std::collections::HashMap;
use diesel::prelude::*;
use diesel::sql_query;
use itertools::Itertools;
use serde::{Serialize, Deserialize};
use regex::Regex;
use log::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct DBSchema<'a> {
	db_name: &'a str,
	db_file: &'a str,
	version: usize,
	#[serde(default)]
	maps: HashMap<String, FileMap>,
	// TODO: include table name in table
	#[serde(default)]
	tables: HashMap<String, Table>,
	#[serde(borrow, default)]
	feeds: HashMap<&'a str, &'a str>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileMap {
	#[serde(default)]
	to_table: Vec<ToTable>,
	#[serde(default)]
	to_keyvalue: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToTable {
	node: String,
	table: String,
	key_col: Option<String>,
	val_col: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Table {
	cols: Vec<(String, String)>,
	indexes: Vec<String>,
	schema_changed: usize,
}

impl Table {
	pub fn to_query(&self, table_name: &str) -> String {
		format!("CREATE TABLE {} ({})",
			table_name,
			self.cols.iter()
				.format_with(", ", |(col, def), f|
					f(&format_args!("{} {}", col, def))
				),
		)
	}
	pub fn insert_statement(&self, object: &serde_json::Value, table_name: &str, conn: &SqliteConnection) -> Result<usize, Option<diesel::result::Error>> {
	// Option<diesel::query_builder::BoxedSqlQuery<diesel::sqlite::Sqlite, diesel::query_builder::SqlQuery>> {
		match object {
			serde_json::Value::Object(map) => {
				let (cols, values): (Vec<_>, Vec<_>) = self.cols.iter()
					.filter_map(|(col, _)| {
						match map.get(col) {
							None | Some(serde_json::Value::Null) => None,
							Some(val) => Some((col, val)), // TODO: don't clone
						}
					})
					.unzip();
				let query = format!("INSERT INTO {} ({}) VALUES ({})",
					table_name,
					cols.iter()
						.format_with(", ", |col, f| f(&format_args!("{}", col))),
						// TODO: properly insert arguments
					values.iter()
						.format_with(", ", |_, f| f(&format_args!("?"))),
				);

				use diesel::sql_types::*;
				use serde_json::Value;
				let query
					= values.iter().fold(sql_query(query).into_boxed::<diesel::sqlite::Sqlite>(), |q, val| {
						match val {
							Value::Number(n) => q.bind::<Integer, _>(n.as_i64().unwrap() as i32),
							Value::String(s) => q.bind::<Text, _>(s),
							Value::Bool(b) => q.bind::<Bool, _>(b),
							Value::Null => q, // TODO: Skip
							_ => q, // TODO: error
						}
					});
				println!("Debug query: {:?}", diesel::debug_query(&query));
				query.execute(conn).map_err(|err| Some(err))
			},
			_ => Err(None),
		}
	}
}

fn create_default_tables<Conn>(conn: &Conn) -> Result<(), diesel::result::Error>
where Conn: diesel::Connection {
	sql_query("CREATE TABLE json (json_id INTEGER PRIMARY KEY ASC NOT NULL UNIQUE);").execute(conn)?;
	sql_query("INSERT INTO json (json_id) VALUES (1);").execute(conn)?;
	// TODO: remove default insert
	
	Ok(())
}

pub fn create_database<Conn>(conn: &Conn, db_schema: &DBSchema) -> Result<(), diesel::result::Error>
where Conn: diesel::Connection {
	create_default_tables(conn)?;
	for (table_name, table) in db_schema.tables.iter() {
		let query = table.to_query(&table_name);
		sql_query(query).execute(conn)?;
	}

	Ok(())
}

pub fn insert_json(conn: &SqliteConnection, schema: &DBSchema, inner_path: &str, json: &HashMap<String, serde_json::Value>) -> Result<(), diesel::result::Error> {
	for (regex, map) in schema.maps.iter() {
		// TODO: prevent panic on bad regex
		let regex = Regex::new(regex).unwrap();
		if regex.is_match(inner_path) {
			println!("Path matches '{}', building queries...", regex);
			json.iter().for_each(|x| println!("{:?}", x));
			for (node, value) in json {
				if let Some(to_table) = map.to_table.iter().find(|x| &x.node == node) {
					println!("Found to_table for '{}'", node);
					if let Some(table) = schema.tables.get(&to_table.table) {
						println!("Table '{}' found: {:?}", &to_table.table, &table);
						if let serde_json::Value::Array(vec) = value {
							for object in vec {
								let result = table.insert_statement(&object, &to_table.table, conn);
								println!("Insert statement result: {:?}", result);
							}
						}
					} else {
						println!("Table '{}' could not be found", &to_table.table);
					}
				}
			}
			println!("{:?}", map.to_table);
			println!("{:?}", map.to_keyvalue);
		}
	}

	Ok(())
}

#[cfg(test)]
mod tests {
	use diesel::sqlite::SqliteConnection;
	use diesel::Connection;
	use super::*;
	use diesel::sql_types::Integer;

	const SCHEMA_STR: &str = r#"
	{
		"db_name": "ZeroTalk",
		"db_file": "data/users/zerotalk.db",
		"version": 2,
		"maps": {
			".+/data.json": {
				"to_table": [
					{"node": "topic", "table": "topic"},
					{"node": "topic_vote", "table": "topic_vote", "key_col": "topic_uri", "val_col": "vote"},
					{"node": "comment", "table": "comment", "key_col": "topic_uri"},
					{"node": "comment_vote", "table": "comment_vote", "key_col": "comment_uri", "val_col": "vote"}
				],
				"to_keyvalue": ["next_comment_id", "next_topic_id"]
			},
			".+/content.json": {
				"to_keyvalue": [ "cert_user_id" ]
			}
		},
		"tables": {
			"topic": {
				"cols": [
					["topic_id", "INTEGER"],
					["title", "TEXT"],
					["body", "TEXT"],
					["type", "TEXT"],
					["parent_topic_uri", "TEXT"],
					["added", "DATETIME"],
					["json_id", "INTEGER REFERENCES json (json_id)"]
				],
				"indexes": ["CREATE UNIQUE INDEX topic_key ON topic(topic_id, json_id)"],
				"schema_changed": 1
			},
			"comment": {
				"cols": [
					["comment_id", "INTEGER"],
					["body", "TEXT"],
					["topic_uri", "TEXT"],
					["added", "DATETIME"],
					["json_id", "INTEGER REFERENCES json (json_id)"]
				],
				"indexes": ["CREATE INDEX topic_uri ON comment(topic_uri)", "CREATE INDEX comment_added ON comment(added)", "CREATE UNIQUE INDEX comment_key ON comment(json_id, comment_id)"],
				"schema_changed": 1
			},
			"comment_vote": {
				"cols": [
					["comment_uri", "TEXT"],
					["vote", "INTEGER"],
					["json_id", "INTEGER REFERENCES json (json_id)"]
				],
				"indexes": ["CREATE UNIQUE INDEX comment_vote_key ON comment_vote(comment_uri, json_id)", "CREATE INDEX comment_vote_uri ON comment_vote(comment_uri)"],
				"schema_changed": 1
			},
			"topic_vote": {
				"cols": [
					["topic_uri", "TEXT"],
					["vote", "INTEGER"],
					["json_id", "INTEGER REFERENCES json (json_id)"]
				],
				"indexes": ["CREATE UNIQUE INDEX topic_vote_topic_key ON topic_vote(topic_uri, json_id)", "CREATE INDEX topic_vote_uri ON topic_vote(topic_uri)"],
				"schema_changed": 1
			}
		},
		"feeds": {
			"Topics": "SELECT title AS title, body AS body, added AS date_added, 'topic' AS type, '?Topic:' || topic.topic_id || '_' || topic_creator_json.directory AS url FROM topic LEFT JOIN json AS topic_creator_json ON (topic_creator_json.json_id = topic.json_id)",
			"Comments": "SELECT 'comment' AS type, comment.added AS date_added, topic.title AS title, commenter_user.value || ': ' || comment.body AS body, topic_creator_json.directory AS topic_creator_address, topic.topic_id || '_' || topic_creator_json.directory AS row_topic_uri, '?Topic:' || topic.topic_id || '_' || topic_creator_json.directory AS url FROM topic LEFT JOIN json AS topic_creator_json ON (topic_creator_json.json_id = topic.json_id) LEFT JOIN comment ON (comment.topic_uri = row_topic_uri) LEFT JOIN json AS commenter_json ON (commenter_json.json_id = comment.json_id) LEFT JOIN json AS commenter_content ON (commenter_content.directory = commenter_json.directory AND commenter_content.file_name = 'content.json') LEFT JOIN keyvalue AS commenter_user ON (commenter_user.json_id = commenter_content.json_id AND commenter_user.key = 'cert_user_id')"
		}
	}"#;

	const JSON_STR: &str = r#"
	{
		"next_topic_id": 2,
		"topic": [
			{
				"topic_id": 1572189025,
				"title": "Test topic group",
				"body": "This is a testing topic group.",
				"added": 1572189024
			},
			{
				"topic_id": 1572189078,
				"title": "Test topic",
				"body": "This is a testing topic.",
				"added": 1572189078,
				"parent_topic_uri": "1572189025"
			}
		],
		"topic_vote": {},
		"next_comment_id": 1,
		"comment": {},
		"comment_vote": {}
	}
	"#;

	#[derive(QueryableByName, PartialEq, Eq, Debug)]
	pub struct Count {
		#[sql_type = "Integer"]
		count: i32,
	}

	fn initialize_database() -> SqliteConnection {
		let conn = SqliteConnection::establish(":memory:").unwrap();

		conn
	}

	#[test]
	fn table_creation() {
		let conn = initialize_database();
		let table = super::Table {
			cols: vec![
				("topic_id".into(), "INTEGER".into()),
				("title".into(), "TEXT".into()),
				("body".into(), "TEXT".into()),
				("type".into(), "TEXT".into()),
				("parent_topic_uri".into(), "TEXT".into()),
				("added".into(), "DATETIME".into()),
				("json_id".into(), "INTEGER REFERENCES json".into()),
			],
			indexes: vec![
				"CREATE UNIQUE INDEX topic_key ON topic(topic_id, json_id);".into(),
			],
			schema_changed: 1,
		};
		let query = table.to_query("test_table");
		println!("Query: {}", query);
		let result = sql_query(&query).execute(&conn);
		assert!(result.is_ok());
		let result = sql_query("INSERT INTO test_table (
				title, body, type, parent_topic_uri, added, json_id
			) VALUES (
				'Title', 'Body', 'Type', 'ParentTopicURI', '2020-01-01', 1
			)").execute(&conn);
		assert!(result.is_ok());
		assert!(result.unwrap() == 1);
		let result = sql_query("SELECT COUNT(*) as count FROM test_table;").load(&conn);
		assert_eq!(Ok(vec![Count{count: 1}]), result, "Count rows");
	}

	#[test]
	fn deserialize_json() {
		let result: Result<DBSchema, _> = serde_json::from_str(SCHEMA_STR);
		println!("{:?}", result);
		assert!(result.is_ok());
	}

	#[test]
	fn database_creation() {
		let conn = initialize_database();
		let db_schema: DBSchema = serde_json::from_str(SCHEMA_STR).unwrap();
		let result = super::create_database(&conn, &db_schema);
		println!("{:?}", result);
		assert!(result.is_ok());
	}

	#[test]
	fn json_insertion() {
		let conn = initialize_database();
		let db_schema: DBSchema = serde_json::from_str(SCHEMA_STR).unwrap();
		super::create_database(&conn, &db_schema);
		let json_object: HashMap<String, serde_json::Value> = serde_json::from_str(JSON_STR).unwrap();
		let result = super::insert_json(&conn, &db_schema, "some/data.json", &json_object);
		assert!(result.is_ok());
		let result = sql_query("SELECT COUNT(*) as count FROM topic;").load(&conn);
		assert_eq!(Ok(vec![Count{count: 2}]), result, "Count rows");
	}
}
