use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
    age: i32,
}

pub fn main() -> Result<()> {
    // 连接到 SQLite 数据库文件
    let conn = Connection::open_in_memory()?;

    // 创建一个表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  age             INTEGER NOT NULL
                  )",
        [],
    )?;

    // 插入一些数据
    conn.execute(
        "INSERT INTO person (name, age) VALUES (?1, ?2)",
        ("Alice", 42),
    )?;

    // 查询数据
    let mut stmt = conn.prepare("SELECT name, age FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok((row.get::<usize, String>(0)?, row.get::<usize, i32>(1)?))
    })?;

    // 输出查询结果
    for person in person_iter {
        let (name, age) = person?;
        // 查看age 数据类型

        println!("{} is {} years old.", name, age);
    }

    Ok(())
}
