use rusqlite::{params, Connection, Result};
use chrono::Utc;

// Public function to set up the DB
pub fn get_db_connection() -> Result<Connection> {
    let home_dir = home::home_dir().expect("Could not find home directory");
    let db_path = home_dir.join(".contextrecall.db");

    let conn = Connection::open(db_path)?;

    conn.execute_batch(
        "BEGIN;
        CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY,
            path TEXT NOT NULL UNIQUE
        );
        CREATE TABLE IF NOT EXISTS commands (
            id INTEGER PRIMARY KEY,
            project_id INTEGER NOT NULL,
            cmd TEXT NOT NULL,
            exit_code INTEGER NOT NULL,
            timestamp INTEGER NOT NULL,
            FOREIGN KEY(project_id) REFERENCES projects(id)
        );
        COMMIT;",
    )?;

    Ok(conn)
}

// Public function to record commands
pub fn record_command(conn: &Connection, project_path: &str, cmd: &str, exit_code: i32) -> Result<()> {
    // If project path is already present in the db it ignores it.
    // else it will create a new record of projects
    conn.execute(
        "INSERT OR IGNORE INTO projects (path) VALUES (?1)",
        params![project_path],
    )?;

    // it fetches the project_id of the project
    let project_id: i32 = conn.query_row(
        "SELECT id FROM projects WHERE path = ?1",
        params![project_path],
        |row| row.get(0),
    )?;

    let timestamp = Utc::now().timestamp();
    // Inserts the command into the commands table w.r.t. the project
    conn.execute(
        "INSERT INTO commands (project_id, cmd, exit_code, timestamp) VALUES (?1, ?2, ?3, ?4)",
        params![project_id, cmd, exit_code, timestamp],
    )?;

    Ok(())
}

// Fetch the recent history for a specific context
pub fn get_context_history(conn: &Connection, project_path: &str) -> Result<Vec<String>> {
    // UPDATED QUERY:
    // We use GROUP BY cmd and ORDER BY MAX(timestamp)
    // This guarantees the most recent run of a command bubbles to the top.
    let mut stmt = conn.prepare(
        "SELECT cmd 
         FROM commands c
         JOIN projects p ON c.project_id = p.id
         WHERE p.path = ?1
         GROUP BY cmd
         ORDER BY MAX(timestamp) DESC
         LIMIT 500"
    )?;

    let command_iter = stmt.query_map(params![project_path], |row| {
        row.get(0)
    })?;

    let mut commands = Vec::new();
    for cmd in command_iter {
        commands.push(cmd?);
    }

    Ok(commands)
}
