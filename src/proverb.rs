use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Proverb {
    pub id: u32,
    pub proverb: String,
    pub meaning: String,
    pub proverb_type: String,
}

impl ToString for Proverb {
    fn to_string(&self) -> String {
        format!(
            "ID: {}\nProverb: {}\nMeaning: {}\nType: {}\n",
            self.id, self.proverb, self.meaning, self.proverb_type
        )
    }
}
