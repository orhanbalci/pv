pub struct Question {
    pub proverb: String,
    pub options: Vec<String>,
    pub correct_meaning: String, // correct meaning
    pub user_answer: String,
}

pub struct Quiz {
    pub questions: Vec<Question>,
    pub score: u32,
}

impl Quiz {
    pub fn new() -> Self {
        Self {
            questions: Vec::new(),
            score: 0,
        }
    }

    pub fn is_finished(&self) -> bool {
        self.questions.iter().all(|q| !q.user_answer.is_empty())
    }

    pub fn calculate_score(&mut self) {
        self.score = self
            .questions
            .iter()
            .fold(0, |acc, q| if q.is_correct() { acc + 1 } else { acc });
    }

    pub fn add_question(&mut self, question: Question) {
        self.questions.push(question);
    }

    pub fn print_score(&self) -> usize {
        println!("")
    }
}

impl Question {
    pub fn new(proverb: String, options: Vec<String>, correct_meaning: String) -> Question {
        Question {
            proverb,
            options,
            correct_meaning,
            user_answer: String::new(),
        }
    }

    pub fn is_correct(&self) -> bool {
        self.user_answer == self.correct_meaning
    }
}
