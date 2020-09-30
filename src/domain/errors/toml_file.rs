use snafu::Snafu;
use serde::export::fmt::Debug;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum FileError {
    #[snafu(display("Could not find environment variable {}: {}", variable, source))]
    EnvVariableError {
        variable: String,
        source: std::env::VarError,
    },
    #[snafu(display("Could not read config to {}: {}", path, source))]
    ReadFile {
        path: String,
        source: std::io::Error,
    },
    #[snafu(display("Could not serialize file {}: {}", path, source))]
    SerializeToml {
        path: String,
        source: toml::ser::Error,
    },
    #[snafu(display("Could not write file to {}: {}", path, source))]
    WriteFile {
        path: String,
        source: std::io::Error,
    },
    #[snafu(display("Could not deserialize file {}: {}", path, source))]
    DeserializeToml {
        path: String,
        source: toml::de::Error,
    },
}