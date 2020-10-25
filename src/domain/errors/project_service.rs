use snafu::Snafu;
use serde::export::fmt::Debug;

use crate::domain::objects::Quantity;
use crate::domain::errors::rustbreak_client::RustbreakClientError;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum ProjectServiceError {
    #[snafu(display("Could not add new task: {}, {}", project_name, task))]
    AddTask {
        project_name: String,
        task: String,
        source: RustbreakClientError
    },
    #[snafu(display("Could not add project: {}", project_name))]
    AddProject {
        project_name: String,
        source: RustbreakClientError
    }
}
