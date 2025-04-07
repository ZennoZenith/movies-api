use std::path::PathBuf;

use crate::utils::{CsvParseError, csv_reader};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Language {
    pub id: u16,
    pub iso_639_1: String,
    pub iso_639_2: String,
    pub language_name_iso_639_2: String,
    pub imdb_language_code: String,
}

pub fn load_language_code() -> Result<Vec<Language>, CsvParseError> {
    let path: PathBuf = "./temp/external/languages_imdb.tsv".into();
    let mut records = Vec::new();

    let mut rdr = csv_reader(&path, false)?;
    for (index, result) in rdr.deserialize::<Language>().enumerate() {
        match result {
            Ok(r) => records.push(r),
            Err(e) => println!("index: {}, error: {:?}", index, e),
        }
    }
    Ok(records)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Country {
    pub id: u16,
    pub name: String,
    pub alpha_3: String,
    pub country_code: String,
    pub iso_3166_2: String,
    pub region: String,
    pub sub_region: String,
    pub intermediate_region: String,
    pub imdb_country_code: String,
}

pub fn load_country_code() -> Result<Vec<Country>, CsvParseError> {
    let path: PathBuf = "./temp/external/countries_imdb.tsv".into();
    let mut records = Vec::new();

    let mut rdr = csv_reader(&path, false)?;
    for (index, result) in rdr.deserialize::<Country>().enumerate() {
        match result {
            Ok(r) => records.push(r),
            Err(e) => println!("index: {}, error: {:?}", index, e),
        }
    }
    Ok(records)
}
