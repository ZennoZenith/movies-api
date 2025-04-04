use std::{fs::File, path::PathBuf};

use csv::{Reader, Writer};
use serde::Serialize;

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

#[derive(thiserror::Error)]
pub enum CsvParseError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    CsvSerialize(#[from] csv::Error),
}

impl std::fmt::Debug for CsvParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

pub fn csv_reader(file_path: &PathBuf, quoteing: bool) -> Result<Reader<File>, CsvParseError> {
    let file = File::open(file_path)?;

    Ok(csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        // .double_quote(false)
        // .escape(Some(b'\\'))
        .quoting(quoteing)
        .flexible(false)
        .comment(Some(b'#'))
        .trim(csv::Trim::All)
        .from_reader(file))
}

pub fn csv_writer(file_path: &PathBuf) -> Result<Writer<File>, CsvParseError> {
    let file = File::create_new(file_path)?;

    Ok(csv::WriterBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .double_quote(false)
        .escape(b'\\')
        .flexible(false)
        .comment(Some(b'#'))
        .quote_style(csv::QuoteStyle::Necessary)
        .from_writer(file))
}

pub fn save_as_csv<T>(file_path: &PathBuf, records: &[T]) -> Result<(), CsvParseError>
where
    T: Serialize,
{
    let mut wtr = csv_writer(file_path)?;
    for (index, value) in records.iter().enumerate() {
        wtr.serialize(value)?;
        if (index + 1) % 10000 == 0 {
            wtr.flush()?;
        }
    }
    wtr.flush()?;

    Ok(())
}
