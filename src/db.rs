use once_cell::sync::Lazy;
use sqlite::Connection;
use std::sync::Mutex;

fn init_db() -> Connection {
    let connection = sqlite::open("database.db").unwrap();

    let query = "
    CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY,        
        username TEXT,
        joined_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );

    CREATE TABLE IF NOT EXISTS flashcards (
        card_id INTEGER PRIMARY KEY AUTOINCREMENT,
        user_id INTEGER NOT NULL,
        question TEXT NOT NULL,
        answer TEXT NOT NULL,
        topic TEXT NOT NULL,
        difficulty INTEGER DEFAULT 0,
        last_quiz_time DATE,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (user_id) REFERENCES users(id)
    );

    CREATE TABLE IF NOT EXISTS quiz_reports (
        quiz_id INTEGER PRIMARY KEY AUTOINCREMENT,
        user_id INTEGER NOT NULL,
        topic TEXT,
        score INTEGER,
        total_questions INTEGER,
        answered_questions INTEGER,
        taken_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        is_over BOOLEAN,
        FOREIGN KEY (user_id) REFERENCES users(id)
    );

    ";
    connection.execute(query).unwrap();
    connection
}

// Singleton DB Access
static DB_CONNECTION: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let conn = init_db();
    Mutex::new(conn)
});

pub fn get_db() -> std::sync::MutexGuard<'static, Connection> {
    DB_CONNECTION.lock().unwrap()
}
