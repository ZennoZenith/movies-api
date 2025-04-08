use std::{
    collections::{BTreeMap, HashSet},
    fs::{File, create_dir_all},
    io::Write,
    path::PathBuf,
    time::Instant,
};

use crate::{
    models::{movies::load_movie_id_tconst, names::load_ids_person},
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
pub struct CrewsDirectors {
    movie_id: u64,
    person_id: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CrewsWriters {
    movie_id: u64,
    person_id: u64,
}

pub fn parse_and_save_imdb_title_crew(file_path: &PathBuf) -> Result<(), CsvParseError> {
    create_dir_all("./temp/parsed/title-crew")
        .expect("cannot create dir: ./temp/parsed/title-crew");

    let mut unknow_movie_tconst = HashSet::new();
    let mut unknow_person_nconst = HashSet::new();

    let ids_movie = load_movie_id_tconst()?;
    let ids_person = load_ids_person()?;

    let ids_movie_map: BTreeMap<&str, u64> = ids_movie
        .iter()
        .map(|data| (data.tconst.as_str(), data.movie_id))
        .collect();

    let ids_person_map: BTreeMap<&str, u64> = ids_person
        .iter()
        .map(|data| (data.nconst.as_str(), data.person_id))
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

        let movie_id = match ids_movie_map.get(record.tconst).copied() {
            Some(id) => id,
            None => {
                unknow_movie_tconst.insert(record.tconst.to_string());
                continue;
            }
        };

        for nconst in record.directors.split(",") {
            if nconst == "\\N" {
                continue;
            }
            match ids_person_map.get(nconst).copied() {
                Some(person_id) => wtr1.serialize(CrewsDirectors {
                    movie_id,
                    person_id,
                })?,
                None => {
                    unknow_person_nconst.insert(nconst.to_string());
                }
            };
        }

        for nconst in record.writers.split(",") {
            if nconst == "\\N" {
                continue;
            }
            match ids_person_map.get(nconst).copied() {
                Some(person_id) => wtr2.serialize(CrewsWriters {
                    movie_id,
                    person_id,
                })?,
                None => {
                    unknow_person_nconst.insert(nconst.to_string());
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
    let v = unknow_movie_tconst.into_iter().collect::<Vec<String>>();
    file.write_all(v.join("\n").as_bytes())?;
    println!(
        "movie id not found for tconst : \n{}",
        v.join("\n").as_str()
    );

    let mut file = File::create(error_write_path2)?;
    let v = unknow_person_nconst.into_iter().collect::<Vec<String>>();
    file.write_all(v.join("\n").as_bytes())?;
    println!("person not found for nconst : \n{}", v.join("\n").as_str());
    Ok(())
}
