use snafu::{ResultExt, IntoError};

use crate::repositories::project_data_repository::ProjectDataRepository;
use crate::domain::objects::{Money, Task};
use crate::domain::errors::project_service::{AddProject, AddTask, ProjectServiceError};
use crate::domain::entities::BillableUnit;

pub struct ProjectService {
    repository: ProjectDataRepository
}

impl ProjectService {
    pub fn new(repository: ProjectDataRepository) -> ProjectService {
        ProjectService {
            repository
        }
    }
    pub fn add_project(&self, project_name: &String, unit_price: Money, unit: BillableUnit) -> Result<(), ProjectServiceError> {
        return self.repository.add_project(project_name, unit_price, unit)
            .map_err(|e|
                AddProject {
                    project_name: project_name.to_string(),
                }.into_error(e)
            );
    }

    pub fn add_task(&self, project_name: &String, task: &Task) -> Result<(), ProjectServiceError> {
        return self.repository.add_task(project_name, task)
            .map_err(|e|
                AddTask {
                    project_name: project_name.to_string(),
                    task: task.to_string(),
                }.into_error(e)
            );
    }

    pub fn sync(&self) {
        let _ = self.repository.write_to_file();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::entities::BillableUnit::Day;

    const PROJECT: &str = "Project";
    const TASK: &str = "Task";

// Run setup function to remove db file before each run:
// https://stackoverflow.com/questions/58006033/how-to-run-setup-code-before-any-tests-run-in-rust

    #[test]
    fn test_add_project() {
        let project = ProjectService::new(
            ProjectDataRepository::new("test_helpers/test_projects.yaml".to_string())
        );

        let _ = project.add_project(&PROJECT.to_string(), 80, Day);

        project.sync()
    }

    #[test]
    fn test_add_task() {
        let project = ProjectService::new(
            ProjectDataRepository::new("test_helpers/test_projects.yaml".to_string())
        );
        let _ = project.add_task(&PROJECT.to_string(), &TASK.to_string());


        project.sync()
    }
}