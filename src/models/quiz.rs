#[derive(Debug)]
pub struct Quiz {
    pub id: f64,
    pub questions: Vec<Question>,
    pub score: usize,
}

#[derive(Debug)]
pub struct Question {
    pub number: u32,
    pub text: String,
    pub answers: Vec<String>,
    pub correct: usize,
}