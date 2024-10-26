use serde::Deserialize;

#[derive(Deserialize)]
pub struct QuestionAnswer {
    pub questions: Vec<String>,
    pub answers: Vec<String>,
}

#[derive(Deserialize)]
pub struct FormDetails {
    pub date: String,
    pub opponent: String,
    pub opponent_details: String,
    pub legal_concern: String,
    pub sender: String,
    pub client: String,
    pub client_details: String,
    pub amount_involved: String,
    pub concern: String,
}




// impl FormDetails {
//     pub fn to_vector(&self) -> Vec<&str> {
//         vec![
//             &self.date_of_letter,
//             &self.receiver_name,
//             &self.receiver_details,
//             &self.subject_of_letter,
//             &self.sender,
//             &self.conforme,
//             &self.content,
//         ]
//     }
// }
