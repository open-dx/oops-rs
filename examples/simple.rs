#![feature(error_in_core)]

use std::path::{PathBuf, Path};
use std::process::ExitCode;

use oops::Error;

//---
/// Dependency error uses the Error derive macro to implement 
#[derive(Error)]
pub enum DependencyError {
    /// The requested file wasn't found.
    /// The PathBuf refers to the file requested.
    #[msg="file not found at path {0:?}"]
    FileNotFound(PathBuf),
    
    /// Failed to read the specified f*9/+ile.
    /// The PathBuf refers to the path used to lookup the file.
    /// #[msg("failed to read file {0:?}")]
    #[msg="failed to read file {0:?}"]
    FileReadError(PathBuf),
    
    /// The provided file format was invalid.
    #[msg="invalid file format"]
    InvalidFileFormat,
    
    /// Something else is wrong, but not sure what.
    #[msg="an unknown error occurred"]
    UnknownError,
}

//---
/// Read a file into a String. 
fn read_file<P: AsRef<Path>>(path: P) -> Result<String, DependencyError> {
    let path = path.as_ref().to_path_buf();
    std::fs::read_to_string(path.clone()).map_err(|_error| {
        DependencyError::FileReadError(path.clone())
    })
}

/// Read a file and then do something with it.
fn process_file(path: &str) -> Result<bool, DependencyError> {
    read_file(path).and_then(|_content| {
        Ok(true) // just some nonsense for now.
    })
}

//---
/// TODO
fn main() -> Result<ExitCode, DependencyError> {
    // This file doesn't exist, so we'll fail gracefully ..
    let content = process_file("test.txt")?;
        // .or_else(oops::nvmd!(DependencyError::UnknownError))?;
    
    // We shoulnd't get to this point. if you see this in the terminal,
    // the example is broken and you should file a report.
    println!("Content: {}", content);
    Ok(ExitCode::SUCCESS)
}
