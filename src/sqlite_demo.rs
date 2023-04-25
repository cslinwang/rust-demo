use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
    age: i32,
}

pub fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;
    // 删除表person
    conn.execute("DROP TABLE IF EXISTS person", [])?;
    conn.execute(
        "CREATE TABLE person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  BLOB,
            age   INT NOT NULL
        )",
        (), // empty list of parameters.
    )?;
    let me: Person = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
        age: 18,
    };
    conn.execute(
        "INSERT INTO person (name, data, age) VALUES (?1, ?2, ?3)",
        (&me.name, &me.data, &me.age),
    )?;
    let mut stmt = conn.prepare("SELECT id, name, data, age FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
            age: row.get(3)?,
        })
    })?;
    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
    Ok(())
}
