use snafu::Snafu;
use serde::export::fmt::Debug;

use crate::domain::objects::Quantity;
use crate::domain::errors::rustbreak_client::RustbreakClientError;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum BillableRepositoryError {
    #[snafu(display("Could not add new billable entry: {}, {}", project_name, task))]
    Create {
        project_name: String,
        task: String,
        source: RustbreakClientError
    }
}
