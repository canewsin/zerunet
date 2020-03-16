#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod models;
mod schema;
mod db_schema;

use models::*;
use schema::*;
use diesel::prelude::*;
use diesel::insert_into;

pub fn create_tables<Conn>(conn: &Conn) -> Result<(), diesel::result::Error> 
  where Conn: diesel::Connection {
  use diesel::sql_query;
  sql_query("CREATE TABLE site (site_id INTEGER PRIMARY KEY ASC NOT NULL UNIQUE,address TEXT NOT NULL);").execute(conn)?;
  sql_query("CREATE UNIQUE INDEX site_address ON site (address);").execute(conn)?;

  sql_query("CREATE TABLE content (content_id INTEGER PRIMARY KEY UNIQUE NOT NULL,site_id INTEGER REFERENCES site (site_id) ON DELETE CASCADE,inner_path TEXT,size INTEGER,size_files INTEGER,size_files_optional INTEGER,modified INTEGER);").execute(conn)?;
  sql_query("CREATE UNIQUE INDEX content_key ON content (site_id, inner_path);").execute(conn)?;
  sql_query("CREATE INDEX content_modified ON content (site_id, modified);").execute(conn)?;

  sql_query("CREATE TABLE peer (site_id INTEGER REFERENCES site (site_id) ON DELETE CASCADE,address TEXT NOT NULL,port INTEGER NOT NULL,hashfield BLOB,reputation INTEGER NOT NULL,time_added INTEGER NOT NULL,time_found INTEGER NOT NULL);").execute(conn)?;
  sql_query("CREATE UNIQUE INDEX peer_key ON peer (site_id, address, port);").execute(conn)?;

  sql_query("CREATE TABLE json (json_id INTEGER PRIMARY KEY AUTOINCREMENT,site VARCHAR(255),directory VARCHAR(255),file_name VARCHAR(255));").execute(conn)?;
  sql_query("CREATE UNIQUE INDEX path ON json(directory, site, file_name);").execute(conn)?;

  sql_query("CREATE TABLE file_optional (file_id INTEGER PRIMARY KEY UNIQUE NOT NULL,site_id INTEGER REFERENCES site (site_id) ON DELETE CASCADE,inner_path TEXT,hash_id INTEGER,size INTEGER,peer INTEGER DEFAULT 0,uploaded INTEGER DEFAULT 0,is_downloaded INTEGER DEFAULT 0,is_pinned INTEGER DEFAULT 0,time_added INTEGER DEFAULT 0,time_downloaded INTEGER DEFAULT 0,time_accessed INTEGER DEFAULT 0);").execute(conn)?;
  sql_query("CREATE UNIQUE INDEX file_optional_key ON file_optional (site_id, inner_path);").execute(conn)?;
  sql_query("CREATE INDEX is_downloaded ON file_optional (is_downloaded);").execute(conn)?;

  sql_query("CREATE TABLE keyvalue (keyvalue_id INTEGER PRIMARY KEY AUTOINCREMENT,key TEXT,value INTEGER,json_id INTEGER);").execute(conn)?;
  sql_query("CREATE UNIQUE INDEX key_id ON keyvalue(json_id, key);").execute(conn)?;

  Ok(())
}

pub fn insert_content(content: zerucontent::Content) -> Result<(), ()> {
  
  Ok(())
}

pub struct ContentManager<Conn: Connection> {
  conn: Conn,
}

impl ContentManager<SqliteConnection> {
  pub fn new(path: &str) -> ContentManager<SqliteConnection> {
    let conn = SqliteConnection::establish(path).unwrap();
    create_tables(&conn);

    ContentManager{
      conn,
    }
  }

  pub fn get_sites(&self) -> Vec<models::Site> {
    site::table.load::<Site>(&self.conn).unwrap()
  }

  pub fn add_site(&self, address: String) -> Result<usize, diesel::result::Error> {
    let new_site = NewSite {
      address: &address,
    };
    insert_into(site::table).values(&new_site).execute(&self.conn)
  }

  pub fn add_content(&self, content: zerucontent::Content) -> Result<usize, diesel::result::Error> {
    let new_content = NewContent::from(content);
    insert_into(content::table).values(&new_content).execute(&self.conn)
  }

  pub fn add_optional(&self, file: zerucontent::File) -> Result<usize, diesel::result::Error> {
    let new_file = NewFileOptional::default();
    insert_into(file_optional::table).values(&new_file).execute(&self.conn)
  }
}

pub fn get_sites(conn: &SqliteConnection) -> Vec<models::Site> {
  let results = site::table.limit(1).load::<Site>(conn).unwrap();
  results
}

#[cfg(test)]
mod tests {
  use diesel::sqlite::SqliteConnection;
  use diesel::Connection;
  use super::*;
  use super::models::*;
  use super::schema::*;

  fn initialize_database() -> SqliteConnection {
    let conn = SqliteConnection::establish(":memory:").unwrap();
    create_tables(&conn).expect("Could not initialize database");

    return conn
  }

  #[test]
  fn table_creation() {
    use diesel::RunQueryDsl;
    use diesel::Connection;
    use diesel::prelude::*;

    let conn = SqliteConnection::establish(":memory:").unwrap();
    assert!(create_tables(&conn).is_ok());
  }

  #[test]
  fn site_insertion() {
    let conn = initialize_database();
    let new_site = NewSite{
      address: "Test",
    };
    assert!(diesel::insert_into(site::table).values(&new_site).execute(&conn).is_ok());
    let results = site::table.limit(1).load::<Site>(&conn).unwrap();
    assert!(results.len() == 1);
    assert!(results[0].address == String::from("Test"));
  }

  #[test]
  fn content_insertion() {
    let conn = initialize_database();
    let mut content = zerucontent::Content::default();
    content.files.insert(String::from("file.json"), zerucontent::File{
      size: 121,
      sha512: String::new(),
    });
    content.modified = 10.0;
    let content = NewContent::from(content);
    assert!(content.size_files == 121);

    assert!(diesel::insert_into(content::table).values(&content).execute(&conn).is_ok());
    let results = content::table.limit(1).load::<Content>(&conn).unwrap();
    assert!(results.len() == 1);
    assert!(results[0].size_files == 121);
  }
}
