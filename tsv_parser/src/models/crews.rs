use std::{
    collections::{BTreeMap, HashSet},
    fs::File,
    io::Write,
    path::PathBuf,
    time::Instant,
};

use crate::{
    models::{movies::load_ids_imdb, names::load_ids_names},
    utils::{CsvParseError, csv_reader, csv_writer},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TitleCrew<'a> {
    /// tconst (string) - alphanumeric unique identifier of the title
    tconst: &'a str,
    /// directors (array of nconsts) - director(s) of the given title
    directors: &'a str,
    /// writers (array of nconsts) – writer(s) of the given title
    writers: &'a str,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CrewsDirectors {
    imdb_id: u64,
    name_id: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CrewsWriters {
    imdb_id: u64,
    name_id: u64,
}

pub fn parse_and_save_imdb_title_crew(file_path: &PathBuf) -> Result<(), CsvParseError> {
    let mut unknow_imdb_id = HashSet::new();
    let mut unknow_name_id = HashSet::new();

    let ids_imdb = load_ids_imdb()?;
    let ids_name = load_ids_names()?;

    let ids_imdb_map: BTreeMap<&str, u64> = ids_imdb
        .iter()
        .map(|data| (data.tconst.as_str(), data.imdb_id))
        .collect();

    let ids_name_map: BTreeMap<&str, u64> = ids_name
        .iter()
        .map(|data| (data.nconst.as_str(), data.name_id))
        .collect();

    let mut rdr = csv_reader(file_path, false)?;
    let mut raw_record = csv::StringRecord::new();
    let headers = rdr.headers()?.clone();

    println!("Parsing {}", file_path.to_str().unwrap_or("invalid path"));

    let write_path1: PathBuf = "./temp/parsed/title-crew/crew_director.tsv".into();
    let write_path2: PathBuf = "./temp/parsed/title-crew/crew_writer.tsv".into();

    let error_write_path1: PathBuf = "./temp/parsed/title-crew/error_tconst.tsv".into();
    let error_write_path2: PathBuf = "./temp/parsed/title-crew/error_nconst.tsv".into();

    let mut wtr1 = csv_writer(&write_path1)?;
    let mut wtr2 = csv_writer(&write_path2)?;

    let mut index: u64 = 1;

    let now = Instant::now();
    while rdr.read_record(&mut raw_record)? {
        let record: TitleCrew = match raw_record.deserialize(Some(&headers)) {
            Ok(r) => r,
            Err(e) => {
                println!("index: {}, error: {:?}", index, e);
                continue;
            }
        };

        let imdb_id = match ids_imdb_map.get(record.tconst).copied() {
            Some(id) => id,
            None => {
                unknow_imdb_id.insert(record.tconst.to_string());
                continue;
            }
        };

        for nconst in record.directors.split(",") {
            if nconst == "\\N" {
                continue;
            }
            match ids_name_map.get(nconst).copied() {
                Some(name_id) => wtr1.serialize(CrewsDirectors { imdb_id, name_id })?,
                None => {
                    unknow_name_id.insert(nconst.to_string());
                }
            };
        }

        for nconst in record.writers.split(",") {
            if nconst == "\\N" {
                continue;
            }
            match ids_name_map.get(nconst).copied() {
                Some(name_id) => wtr2.serialize(CrewsWriters { imdb_id, name_id })?,
                None => {
                    unknow_name_id.insert(nconst.to_string());
                }
            };
        }

        if index % 10000 == 0 {
            wtr1.flush()?;
            wtr2.flush()?;
        }

        index += 1;
    }
    wtr1.flush()?;
    wtr2.flush()?;

    let elapsed_time = now.elapsed();
    println!(
        "Done parsing {} in {} ms.",
        file_path.to_str().unwrap_or("invalid path"),
        elapsed_time.as_millis()
    );

    let mut file = File::create(error_write_path1)?;
    let v = unknow_imdb_id.into_iter().collect::<Vec<String>>();
    file.write_all(v.join("\n").as_bytes())?;
    println!(
        "imdb id not found for title_id/tconst : \n{}",
        v.join("\n").as_str()
    );

    let mut file = File::create(error_write_path2)?;
    let v = unknow_name_id.into_iter().collect::<Vec<String>>();
    file.write_all(v.join("\n").as_bytes())?;
    println!(
        "imdb id not found for title_id/tconst : \n{}",
        v.join("\n").as_str()
    );
    Ok(())
}
