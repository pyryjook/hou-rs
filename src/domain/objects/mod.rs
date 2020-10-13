use::serde::{Serialize, Deserialize};


pub type Money = u16;
pub type Quantity = f32;
pub type Task = String;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Config {
    pub lex_office_api_key: Option<String>
}