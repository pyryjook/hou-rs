use chrono::{DateTime, Local};
use snafu::{ResultExt, IntoError};

use crate::clients::rustbreak_client::ProjectDataClient;
use crate::domain::objects::{Quantity, Money, Task};
use crate::domain::errors::billable_repository::{Create, BillableRepositoryError};
use crate::domain::entities::{BillableUnit, Billable};

pub struct BillableRepository {
    client: ProjectDataClient
}

impl BillableRepository {
    pub fn new(client: ProjectDataClient) -> BillableRepository {
        BillableRepository {
            client
        }
    }

    pub fn create(&self, Billable { project, task, quantity, date, .. }: Billable) -> Result<(), BillableRepositoryError> {
        return self.client.add_billable_entry(&project, &task, quantity, Some(date))
            .map_err(|e|
                Create {
                    project_name: project_name.to_string(),
                    task: task.to_string()
                }.into_error(e)
            );
    }

    pub fn sync(&self) {
        let _ = self.client.write_to_file();
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
    fn test_new_entry() {
        let billable = BillableRepository::new(
            ProjectDataClient::new("test_helpers/test_billable.yaml".to_string())
        );

        let res = billable.create(&PROJECT.to_string(), &TASK.to_string(), 5.0, None).unwrap();

        billable.sync()
    }
}
