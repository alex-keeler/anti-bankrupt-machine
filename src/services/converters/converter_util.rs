use csv::StringRecord;
use std::format;
use crate::models::data::application_error::ApplicationError;

pub fn verify_headers(header_record: &StringRecord, expected_headers: Box<&[&'static str]>) -> Result<(), ApplicationError> {

    for (index, expected_header) in expected_headers.iter().enumerate() {
        let header = header_record.get(index).ok_or(format!("Header not found at index {}", index))?;
        if header != *expected_header {
            return Err(format!("Incorrect header: Expected [{}] at index {}, got [{}] instead", *expected_header, index, header).into());
        }
    }

    Ok(())
}