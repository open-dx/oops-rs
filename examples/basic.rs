#![feature(error_in_core)]

use std::path::{PathBuf, Path};
use std::process::ExitCode;

//---
/// Dependency error uses the Error derive macro to implement 
#[derive(oops::Error)]
pub enum DependencyError {
    /// The requested file wasn't found.
    /// The PathBuf refers to the file requested.
    #[msg="file not found at path {0:?}"]
    FileNotFound(PathBuf),
    
    /// The provided file format was invalid.
    #[msg="invalid file format"]
    InvalidFileFormat,
    
    /// Something else is wrong, but not sure what.
    #[msg="config error; {:}"]
    ConfigError(ConfigError),
    
    /// Something else is wrong, but not sure what.
    #[msg="the operation failed successfully <3"]
    RazzleDazzle,
    
    /// Something else is wrong, but not sure what.
    #[msg="an unknown error occurred"]
    UnknownError,
}

/// TODO
#[derive(oops::Error)]
pub enum ConfigError {
    /// Failed to read the specified f*9/+ile.
    /// The PathBuf refers to the path used to lookup the file.
    /// #[msg("failed to read file {0:?}")]
    #[msg="failed to read file {0:?}"]
    FileReadError(PathBuf),

    /// Something else is wrong, but not sure what.
    #[msg="an unknown error occurred"]
    UnknownError,
}

//---
/// Read a file into a String. 
fn read_config<P: AsRef<Path>>(path: P) -> Result<String, ConfigError> {
    let path = path.as_ref().to_path_buf();
    std::fs::read_to_string(path.clone()).map_err(|_error| {
        ConfigError::FileReadError(path.clone())
    })
}

/// Read a file and then do something with it.
fn setup_runtime(path: &str) -> Result<bool, DependencyError> {
    // Simulate some i/o operation ..
    read_config(path)
        .or_else(|error| {
            // TODO: Move into an oops helper:
            //   - Input should be just the final error.
            tracing::trace!("Configuration Error: {:}", error);
            Err(DependencyError::ConfigError(error))
        })
        .and_then(|_content| {
            // Just some nonsense for now.
            Ok(true)
        })
}

//---
/// TODO
fn main() -> Result<ExitCode, DependencyError> {
    // This file doesn't exist, so we'll fail gracefully ..
    let content = setup_runtime("test.txt") // ?;
        // TODO: Unwrap errors into inner bois.
        .or_else(oops::nvmd![DependencyError::RazzleDazzle])?;
    
    // We shoulnd't get to this point. if you see this in the terminal,
    // the example is broken and you should file a report.
    println!("Content: {}", content);
    Ok(ExitCode::SUCCESS)
}
