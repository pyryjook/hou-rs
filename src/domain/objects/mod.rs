use::serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub lex_office_api_key: Option<String>
}