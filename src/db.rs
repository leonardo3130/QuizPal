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
        interval_days INTEGER DEFAULT 1,
        next_review_date DATE DEFAULT CURRENT_DATE,
        times_reviewed INTEGER DEFAULT 0,
        last_reviewed DATE,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (user_id) REFERENCES users(id)
    );

    CREATE TABLE IF NOT EXISTS quizzes (
        quiz_id INTEGER PRIMARY KEY AUTOINCREMENT,
        user_id INTEGER NOT NULL,
        topic TEXT,
        score INTEGER,
        total_questions INTEGER,
        taken_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (user_id) REFERENCES users(id)
    );

    CREATE TABLE IF NOT EXISTS review_logs (
        log_id INTEGER PRIMARY KEY AUTOINCREMENT,
        user_id INTEGER NOT NULL,
        card_id INTEGER NOT NULL,
        was_correct BOOLEAN,
        reviewed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (user_id) REFERENCES users(id),
        FOREIGN KEY (card_id) REFERENCES flashcards(card_id)
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
