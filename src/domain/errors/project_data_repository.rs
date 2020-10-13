use snafu::Snafu;
use serde::export::fmt::Debug;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum ProjectDataRepositoryError {
    #[snafu(display("Could not read data from db: {}", source))]
    ReadFailed {
        source: rustbreak::RustbreakError,
    },
    #[snafu(display("Could not write data to db: {}", source))]
    WriteFailed {
        source: rustbreak::RustbreakError,
    },
    #[snafu(display("Unexpected task: {}", task))]
    UnexpectedTask {
        task: String
    },
    #[snafu(display("Could save data to file: {}", source))]
    SaveToFile {
        source: rustbreak::RustbreakError,
    },
    #[snafu(display("Malformed date string"))]
    MalformedDateString,
    #[snafu(display("Could not read Billable, is the data malformed?"))]
    NoneError,
}