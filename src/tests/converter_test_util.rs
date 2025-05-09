use std::io::Write;
use tempfile::NamedTempFile;
use crate::models::data::application_error::ApplicationError;

pub fn build_temp_file(lines: Vec<&str>) -> Result<NamedTempFile, ApplicationError> {

    let mut temp_file = NamedTempFile::new().or(Err("could not create temp file"))?;

    for line in lines {
        temp_file.write(format!("{}\n", line).as_bytes()).or(Err(format!("could not write line [{:?}] to temp file", line)))?;
    }

    Ok(temp_file)
}