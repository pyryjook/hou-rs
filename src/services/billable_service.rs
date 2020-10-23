use chrono::{DateTime, Local};
use snafu::{ResultExt, IntoError};

use crate::repositories::project_data_repository::ProjectDataRepository;
use crate::domain::objects::{Quantity, Money, Task};
use crate::domain::errors::billable_service::{AddProject, AddTask, NewEntry, BillableServiceError};
use crate::domain::entities::BillableUnit;

pub struct BillableService {
    repository: ProjectDataRepository
}

impl BillableService {
    pub fn new(repository: ProjectDataRepository) -> BillableService {
        BillableService {
            repository
        }
    }

    pub fn add_project(&self, project_name: &String, unit_price: Money, unit: BillableUnit) -> Result<(), BillableServiceError> {
        return self.repository.add_project(project_name, unit_price, unit)
            .map_err(|e|
                AddProject {
                    project_name: project_name.to_string(),
                }.into_error(e)
            );
    }

    pub fn add_task(&self, project_name: &String, task: &Task) -> Result<(), BillableServiceError> {
        return self.repository.add_task(project_name, task)
            .map_err(|e|
                AddTask {
                    project_name: project_name.to_string(),
                    task: task.to_string(),
                }.into_error(e)
            );
    }

    pub fn new_entry(&self, project_name: &String, task: &String, quantity: Quantity, date: Option<DateTime<Local>>) -> Result<(), BillableServiceError> {
        return self.repository.add_billable_entry(project_name, task, quantity, date)
            .map_err(|e|
                NewEntry {
                    project_name: project_name.to_string(),
                    task: task.to_string()
                }.into_error(e)
            );
    }

    pub fn sync(&self) {
        let _ = self.repository.write_to_file();
    }
}

#[cfg(test)]
mod test
    use super::*;
    use crate::domain::entities::BillableUnit::Day;

    const PROJECT: &str = "Project";
    const TASK: &str = "Task";

// Run setup function to remove db file before each run:
// https://stackoverflow.com/questions/58006033/how-to-run-setup-code-before-any-tests-run-in-rust

    #[test]
    fn test_add_project() {
        let billable = BillableService::new(
            ProjectDataRepository::new("test_helpers/test_billable.yaml".to_string())
        );

        let _ = billable.add_project(&PROJECT.to_string(), 80, Day);

        billable.sync()
    }

    #[test]
    fn test_add_task() {
        let billable = BillableService::new(
            ProjectDataRepository::new("test_helpers/test_billable.yaml".to_string())
        );
        let _ = billable.add_task(&PROJECT.to_string(), &TASK.to_string());


        billable.sync()
    }

    #[test]
    fn test_new_entry() {
        let billable = BillableService::new(
            ProjectDataRepository::new("test_helpers/test_billable.yaml".to_string())
        );

        let res = billable.new_entry(&PROJECT.to_string(), &TASK.to_string(), 5.0, None).unwrap();

        billable.sync()
    }
}