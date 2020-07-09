#[derive(Debug)]
/// A multiple choice knowledge test
pub struct Quiz {
    pub id: f64,
    pub questions: Vec<Question>,
    pub score: usize,
}

#[derive(Debug)]
/// A question in a knowledge test
pub struct Question {
    pub number: u32,
    pub question_text: String,
    pub answers: Vec<String>,
    pub correct_answer: usize,
}