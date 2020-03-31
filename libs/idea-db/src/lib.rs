use std::path::Path;
use std::time::Duration;

use rusqlite::{
    backup::{Backup, Progress},
    params, Result, Connection, DatabaseName, OpenFlags
};

pub mod key;
pub mod types;

use key::Key;

pub struct IdeaDatabase {
    conn: Connection
}

impl IdeaDatabase {
    pub fn new() -> Result<IdeaDatabase> {
        let mut db = IdeaDatabase {
            conn: Connection::open_in_memory()?
        };

        create_database(&mut db.conn)?;
        Ok(db)
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<IdeaDatabase> {
        let mut db = IdeaDatabase::new()?;

        {
            let src = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
            let backup = Backup::new(&src, &mut db.conn)?;
            backup.run_to_completion(5, Duration::from_millis(250), None)?;
        }

        Ok(db)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        self.conn.backup(DatabaseName::Main, path, None)
    }

    pub fn add_thing_kind(&mut self, kind: &types::ThingKind) -> Result<Key<types::ThingKind>> {
        let mut statement = self.conn.prepare_cached("INSERT INTO thing_kind(name) VALUES(?);")?;
        let id = statement.insert(params![&kind.name])?;
        Ok(Key::new(id))
    }

    pub fn add_connection_kind(&mut self, kind: &types::ConnectionKind) -> Result<Key<types::ConnectionKind>> {
        let mut statement = self.conn.prepare_cached("INSERT INTO connection_kind(name, lhs, rhs) VALUES(?, ?, ?);")?;
        let id = statement.insert(params![&kind.name, kind.lhs.key, kind.rhs.key])?;
        Ok(Key::new(id))
    }

    pub fn add_thing(&mut self, thing: &types::Thing) -> Result<Key<types::Thing>> {
        let mut statement = self.conn.prepare_cached("INSERT INTO thing(kind, name, x, y) VALUES(?, ?, ?, ?);")?;
        let id = statement.insert(params![thing.kind.key, &thing.name, thing.x, thing.y])?;
        Ok(Key::new(id))
    }

    pub fn add_connection(&mut self, connection: &types::Connection) -> Result<Key<types::Connection>> {
        let mut statement = self.conn.prepare_cached("INSERT INTO connection(kind, lhs, rhs) VALUES(?, ?, ?);")?;
        let id = statement.insert(params![connection.kind.key, connection.lhs.key, connection.rhs.key])?;
        Ok(Key::new(id))
    }

    pub fn get_thing_info(&mut self) -> Result<Vec<types::FullThing>> {
        self.conn
            .prepare_cached(
                "SELECT
                    thing.id,
                    thing.kind,
                    thing.name,
                    thing.x,
                    thing.y,
                    thing_kind.name kind_name
                FROM
                    thing JOIN thing_kind ON thing.kind = thing_kind.id;")?
            .query_map(
                params![],
                |row| Ok(types::FullThing {
                    key: Key::new(row.get("id")?),
                    thing: types::Thing {
                        kind: Key::new(row.get("kind")?),
                        name: row.get("name")?,
                        x: row.get("x")?,
                        y: row.get("y")?
                    },
                    kind: types::ThingKind {
                        name: row.get("kind_name")?
                    }
                }))?
            .collect()
    }

    pub fn get_connection_info(&mut self) -> Result<Vec<types::FullConnection>> {
        self.conn
            .prepare_cached(
                "SELECT
                    connection.id,
                    connection.kind,
                    connection.lhs,
                    connection.rhs,
                    connection_kind.name kind_name,
                    connection_kind.lhs lhs_kind,
                    connection_kind.rhs rhs_kind
                FROM
                    connection JOIN connection_kind ON connection.kind = connection_kind.id;")?
            .query_map(
                params![],
                |row| Ok(types::FullConnection {
                    key: Key::new(row.get("id")?),
                    connection: types::Connection {
                        kind: Key::new(row.get("kind")?),
                        lhs: Key::new(row.get("lhs")?),
                        rhs: Key::new(row.get("rhs")?)
                    },
                    kind: types::ConnectionKind {
                        name: row.get("kind_name")?,
                        lhs: Key::new(row.get("lhs_kind")?),
                        rhs: Key::new(row.get("rhs_kind")?),
                    }
                }))?
            .collect()
    }
}

fn create_database(conn: &mut Connection) -> Result<()>
{
    conn.execute(
        "CREATE TABLE thing_kind(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT);",
        params![])?;

    conn.execute(
        "CREATE TABLE connection_kind(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            lhs INTEGER NOT NULL REFERENCES thing_kind(id),
            rhs INTEGER NOT NULL REFERENCES thing_kind(id));",
        params![])?;

    conn.execute(
        "CREATE TABLE thing(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            kind INTEGER NOT NULL REFERENCES thing_kind(id),
            name TEXT,
            x INTEGER,
            y INTEGER);",
        params![])?;

    conn.execute(
        "CREATE TABLE connection(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            kind INTEGER NOT NULL REFERENCES connection_kind(id),
            lhs INTEGER NOT NULL REFERENCES thing(id),
            rhs INTEGER NOT NULL REFERENCES thing(id),

            UNIQUE(kind, lhs, rhs));",
        params![])?;

    conn.execute(
        "CREATE VIEW valid_connections AS
        WITH
            all_possibilities AS (
                SELECT
                    lhs.id AS lhs_id,
                    rhs.id AS rhs_id,
                    lhs.kind AS lhs,
                    rhs.kind AS rhs
                FROM
                    thing AS lhs,
                    thing AS rhs)
        SELECT
            *
        FROM
            all_possibilities
                JOIN connection_kind
                USING(lhs, rhs);",
        params![])?;

    conn.execute(
        "CREATE TRIGGER validate_connection
        AFTER INSERT ON connection
        WHEN NOT EXISTS (SELECT * FROM valid_connections vc WHERE vc.lhs_id = NEW.lhs AND vc.rhs_id = NEW.rhs)
        BEGIN
            SELECT RAISE(FAIL, \"Invalid connection\");
        END;",
        params![])?;

    Ok(())
}
