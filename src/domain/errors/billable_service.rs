use snafu::Snafu;
use serde::export::fmt::Debug;

use crate::domain::objects::Quantity;
use crate::domain::errors::project_data_repository::ProjectDataRepositoryError;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum BillableServiceError {
    #[snafu(display("Could not add new billable entry: {}, {}", project_name, task))]
    NewEntry {
        project_name: String,
        task: String,
        source: ProjectDataRepositoryError
    }
}
