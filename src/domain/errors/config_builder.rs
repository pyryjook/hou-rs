use snafu::{self, Snafu};

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum ConfigBuilderError {
    #[snafu(display("Could not build config, did you remember to call correct builder methods?"))]
    NoneError,
}