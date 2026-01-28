use rusqlite::Connection;

pub struct AppState {
    pub conn: std::sync::Mutex<Connection>,
}

pub fn init() -> rusqlite::Result<Connection> {
    let conn = Connection::open("database/app.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS master_content (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            page_name TEXT NOT NULL,
            section_name TEXT NOT NULL,
            lang TEXT NOT NULL,
            content_type TEXT NOT NULL,
            content TEXT NOT NULL,
            visible INTEGER DEFAULT 1,
            display_order INTEGER DEFAULT 0,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;
    Ok(conn)
}
