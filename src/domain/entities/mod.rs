use::serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};
use std::collections::HashSet;
use crate::domain::objects::{Quantity, Money, Task};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum BillableUnit {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "hour")]
    Hour
}

#[derive(Debug, PartialEq)]
pub struct Billable {
    pub project_id: String,
    pub task: String,
    pub quantity: Quantity,
    pub date: DateTime<Local>
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Project {
    pub name: String,
    pub unit_price: Money,
    pub unit: BillableUnit,
    pub tasks: HashSet<Task>
}