use crate::db;

use sqlite::State;

#[derive(Clone)]
pub struct FlashCardData {
    pub difficulty: i64,
    pub answer: String,
    pub question: String,
}

#[derive(Default, Clone)]
pub struct QuizData {
    pub user_id: u64,
    pub topic: String,
}

#[derive(Default, Clone)]
pub struct QuizManager {
    info: QuizData,
    current: usize,
    score: usize,
    total_questions: usize,
    answered_questions: usize,
    cards: Vec<FlashCardData>,
    new: bool,
}

impl QuizManager {
    pub fn new(info: QuizData) -> Self {
        let cards: Vec<FlashCardData> = {
            let db = db::get_db();
            let mut statement = db
                .prepare(
                    "
                    SELECT question, answer, difficulty
                    FROM flashcards
                    WHERE topic = ? AND user_id = ?
                    ORDER BY difficulty
                                ",
                )
                .unwrap();

            statement.bind((1, info.topic.as_str())).unwrap();
            statement
                .bind((2, info.user_id.to_string().as_str()))
                .unwrap();

            let mut rows: Vec<FlashCardData> = Vec::new();

            while let Ok(State::Row) = statement.next() {
                rows.push(FlashCardData {
                    question: statement.read::<String, _>("question").unwrap(),
                    answer: statement.read::<String, _>("question").unwrap(),
                    difficulty: statement.read::<i64, _>("difficulty").unwrap(),
                });
            }

            rows
        };

        Self {
            info: info,
            current: 0,
            score: 0,
            total_questions: cards.len(),
            answered_questions: 0,
            cards: cards,
            new: true,
        }
    }

    pub fn get_question(&mut self) -> Result<Option<&FlashCardData>, sqlite::Error> {
        if self.current >= self.cards.len() {
            return match self.save_quiz_result() {
                Ok(_) => Ok(None),
                Err(e) => Err(e),
            };
        }
        let q = Ok(self.cards.get(self.current));
        self.current += 1;
        q
    }

    pub fn check_answer(&mut self, input: &str) -> bool {
        let is_correct = input == self.cards.get(self.current).unwrap().answer;
        self.answered_questions += 1;
        if is_correct {
            self.score += 1;
        }
        is_correct
    }

    pub fn save_quiz_result(&mut self) -> Result<(), sqlite::Error> {
        let db = db::get_db();
        let mut statement = db
            .prepare(
                "
                INSERT INTO quiz_reports (
                    user_id,
                    topic,
                    score,
                    total_questions,
                    answered_questions
                ) VALUES (
                    ?,
                    ?,
                    ?,
                    ?,
                    ?
                );",
            )
            .unwrap();

        statement
            .bind((1, self.info.user_id.to_string().as_str()))
            .unwrap();
        statement.bind((2, self.info.topic.as_str())).unwrap();
        statement
            .bind((3, self.score.to_string().as_str()))
            .unwrap();
        statement
            .bind((4, self.total_questions.to_string().as_str()))
            .unwrap();
        statement
            .bind((5, self.answered_questions.to_string().as_str()))
            .unwrap();

        statement.next()?;
        Ok(())
    }

    pub fn get_score(&mut self) -> usize {
        self.score
    }

    pub fn get_answered(&mut self) -> usize {
        self.answered_questions
    }

    pub fn is_new(&mut self) -> bool {
        let prev = self.new;
        if prev {
            self.new = false
        }
        prev
    }
}
