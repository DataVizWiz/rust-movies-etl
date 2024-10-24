use csv::{Error, Reader, ReaderBuilder, Trim};
use std::fs::File;

pub fn read_csv(file: &str) -> Result<Reader<File>, Error> {
    let mut builder = ReaderBuilder::new();

    builder.delimiter(b',').has_headers(true).trim(Trim::All);

    let result = builder.from_path(file)?;
    Ok(result)
}
