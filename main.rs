use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use std::env;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};
use anyhow::Result;
use rusqlite::{Connection, params};

#[derive(Serialize, Deserialize, Debug)]
pub struct Snippet {
    pub content: String,
    pub created_at: OffsetDateTime,
}

pub trait SnippetStorage {
    fn load(&mut self) -> Result<HashMap<String, Snippet>>;
    fn save(&mut self, data: &HashMap<String, Snippet>) -> Result<()>;
}

pub struct JsonStorage {
    path: String,
}

impl JsonStorage {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

impl SnippetStorage for JsonStorage {
    fn load(&mut self) -> Result<HashMap<String, Snippet>> {
        if let Ok(data) = fs::read_to_string(&self.path) {
            let map = serde_json::from_str(&data)?;
            Ok(map)
        } else {
            Ok(HashMap::new())
        }
    }

    fn save(&mut self, data: &HashMap<String, Snippet>) -> Result<()> {
        let json = serde_json::to_string_pretty(data)?;
        fs::write(&self.path, json)?;
        Ok(())
    }
}

pub struct SqliteStorage {
    conn: Connection,
}

impl SqliteStorage {
    pub fn new(path: String) -> Result<Self> {
        let conn = Connection::open(path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS snippets (
                name TEXT PRIMARY KEY,
                content TEXT NOT NULL,
                created_at TEXT NOT NULL
            )",
            [],
        )?;

        Ok(Self { conn })
    }
}

impl SnippetStorage for SqliteStorage {
    fn load(&mut self) -> Result<HashMap<String, Snippet>> {
        let mut stmt = self.conn.prepare(
            "SELECT name, content, created_at FROM snippets"
        )?;

        let rows = stmt.query_map([], |row| {
            let name: String = row.get(0)?;
            let content: String = row.get(1)?;
            let created_at_str: String = row.get(2)?;
            let created_at = OffsetDateTime::parse(&created_at_str, &Rfc3339).unwrap();

            Ok((name, Snippet { content, created_at }))
        })?;

        let mut map = HashMap::new();
        for row in rows {
            let (name, snippet) = row?;
            map.insert(name, snippet);
        }

        Ok(map)
    }

    fn save(&mut self, data: &HashMap<String, Snippet>) -> Result<()> {
        self.conn.execute("DELETE FROM snippets", [])?;

        for (name, snippet) in data {
            self.conn.execute(
                "INSERT INTO snippets (name, content, created_at) VALUES (?, ?, ?)",
                params![
                    name,
                    snippet.content,
                    snippet.created_at.format(&Rfc3339)?
                ],
            )?;
        }

        Ok(())
    }
}



fn init_storage() -> Box<dyn SnippetStorage> {
    let config = env::var("SNIPPETS_APP_STORAGE")
        .expect("SNIPPETS_APP_STORAGE not set");

    let parts: Vec<&str> = config.split(':').collect();
    if parts.len() != 2 {
        panic!("SNIPPETS_APP_STORAGE must be like JSON:path or SQLITE:path");
    }

    let provider = parts[0];
    let path = parts[1].to_string();

    match provider {
        "JSON" => Box::new(JsonStorage::new(path)),
        "SQLITE" => Box::new(SqliteStorage::new(path).unwrap()),
        _ => panic!("Unknown storage provider: {provider}")
    }
}


fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut storage = init_storage();
    let mut map = storage.load()?;

    if args.len() >= 3 && args[1] == "--name" {
        let name = args[2].clone();

        let mut content = String::new();
        io::stdin().read_to_string(&mut content)?;

        let snippet = Snippet {
            content,
            created_at: OffsetDateTime::now_utc(),
        };

        map.insert(name, snippet);
        storage.save(&map)?;
        println!("Snippet saved.");
    }
    else if args.len() >= 3 && args[1] == "--read" {
        let name = &args[2];
        if let Some(sn) = map.get(name) {
            println!("Created at: {}", sn.created_at.format(&Rfc3339)?);
            println!("{}", sn.content);
        } else {
            println!("Snippet not found.");
        }
    }
    else if args.len() >= 3 && args[1] == "--delete" {
        let name = &args[2];
        if map.remove(name).is_some() {
            storage.save(&map)?;
            println!("Snippet deleted.");
        } else {
            println!("Snippet not found.");
        }
    }
    else {
        println!("Usage:");
        println!("  --name <name>   (read snippet from stdin and save)");
        println!("  --read <name>");
        println!("  --delete <name>");
    }

    Ok(())
}