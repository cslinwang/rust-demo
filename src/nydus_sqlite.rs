use once_cell::sync::Lazy;
use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};

// chunk
struct Chunk {
    blob_id: String,
    chunk_digest: String,
    chunk_compressed_size: String,
}

impl Chunk {
    fn new(blob_id: String, chunk_digest: String, chunk_compressed_size: String) -> Chunk {
        Chunk {
            blob_id: blob_id,
            chunk_digest: chunk_digest,
            chunk_compressed_size: chunk_compressed_size,
        }
    }
}

// blob
struct Blob {
    blob_id: String,
    blob_size: String,
    image_name: String,
}

static DB: Lazy<Arc<Mutex<Connection>>> = Lazy::new(|| {
    let db = Connection::open("my_database.db").expect("Failed to open database");
    setup_tables(&db).expect("Failed to create tables");
    Arc::new(Mutex::new(db))
});

fn setup_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS chunk (
            blob_id TEXT NOT NULL,
            chunk_digest TEXT NOT NULL,
            chunk_compressed_size TEXT NOT NULL,
            PRIMARY KEY (chunk_digest)
        );
        CREATE TABLE IF NOT EXISTS blob (
            blob_id TEXT NOT NULL PRIMARY KEY,
            blob_size TEXT NOT NULL,
            image_name TEXT NOT NULL
        );
        "#,
    )
}

fn perform_query() -> Result<()> {
    let conn = DB.lock().unwrap();
    // 插入数据
    let chunk: Chunk = Chunk::new(
        "blob_id".to_string(),
        "chunk_digest".to_string(),
        "chunk_compressed_size".to_string(),
    );

    // conn.execute(
    //     "INSERT INTO chunk (blob_id, chunk_digest, chunk_compressed_size) VALUES (?1, ?2, ?3)",
    //     &[
    //         chunk.blob_id,
    //         chunk.chunk_digest,
    //         &chunk.chunk_compressed_size,
    //     ],
    // )?;

    // 查询数据
    let mut stmt: rusqlite::Statement =
        conn.prepare("SELECT blob_id, chunk_digest, chunk_compressed_size FROM chunk")?;
    let chunk_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<usize, String>(0)?,
            row.get::<usize, String>(1)?,
            row.get::<usize, String>(2)?,
        ))
    })?;
    // 输出查询结果
    for chunk in chunk_iter {
        let (blob_id, chunk_digest, chunk_compressed_size) = chunk?;
        println!(
            "blob_id: {}, chunk_digest: {}, chunk_compressed_size: {}",
            blob_id, chunk_digest, chunk_compressed_size
        );
    }
    Ok(())
}

pub fn main() {
    // This line is required to initialize the static DB connection.
    let _ = &*DB;
    let _ = perform_query();
}
