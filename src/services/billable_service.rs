use crate::repositories::project_data_repository::ProjectDataRepository;
use crate::domain::objects::Quantity;
use chrono::{DateTime, Local};

pub struct BillableService {
    repository: ProjectDataRepository
}

impl BillableService {
    pub fn new(repository: ProjectDataRepository) -> BillableService {
        BillableService {
            repository
        }
    }

    pub fn add(&self, project_name: &String, task: &String, quantity: Quantity, date: Option<DateTime<Local>>) {
        self.repository.add_billable_entry(project_name, task, quantity, date)
    }
}