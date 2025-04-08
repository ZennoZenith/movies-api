use std::{
    collections::{BTreeMap, HashSet},
    fs::{File, create_dir_all},
    io::Write,
    path::PathBuf,
    time::Instant,
};

use crate::{
    models::{
        movies::load_movie_id_tconst,
        names::{load_ids_person, load_ids_professions},
    },
    utils::{CsvParseError, csv_reader, csv_writer, save_as_csv},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TitlePrincipals<'a> {
    /// tconst (string) - alphanumeric unique identifier of the title
    tconst: &'a str,
    /// ordering (integer) – a number to uniquely identify rows for a given titleId
    ordering: u16,
    /// nconst (string) - alphanumeric unique identifier of the name/person
    nconst: &'a str,
    /// category (string) - the category of job that person was in
    category: &'a str,
    /// job (string) - the specific job title if applicable, else '\N'
    job: &'a str,
    /// characters (string) - the name of the character played if applicable, else '\N'
    characters: &'a str,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Principals<'a> {
    movie_id: u64,
    ordering: u16,
    person_id: u64,
    profession_id: u16, // <- category,
    job: &'a str,
    characters: &'a str,
}

pub fn parse_and_save_title_principal(file_path: &PathBuf) -> Result<(), CsvParseError> {
    create_dir_all("./temp/parsed/title-principal")
        .expect("cannot create dir: ./temp/parsed/title-principal");

    let mut unknow_movie_tconst = HashSet::new();
    let mut unknow_person_nconst = HashSet::new();
    let mut unknow_profession_id = HashSet::new();

    let ids_movie = load_movie_id_tconst()?;
    let ids_person = load_ids_person()?;
    let ids_profession = load_ids_professions()?;

    let ids_movie_map: BTreeMap<&str, u64> = ids_movie
        .iter()
        .map(|data| (data.tconst.as_str(), data.movie_id))
        .collect();

    let ids_person_map: BTreeMap<&str, u64> = ids_person
        .iter()
        .map(|data| (data.nconst.as_str(), data.person_id))
        .collect();

    let ids_profession_map: BTreeMap<&str, u16> = ids_profession
        .iter()
        .map(|data| (data.profession.as_str(), data.id))
        .collect();

    let mut rdr = csv_reader(file_path, false)?;
    let mut raw_record = csv::StringRecord::new();
    let headers = rdr.headers()?.clone();

    println!("Parsing {}", file_path.to_str().unwrap_or("invalid path"));

    let write_path1: PathBuf = "./temp/parsed/title-principal/principals.tsv".into();
    let write_path2: PathBuf = "./temp/parsed/title-principal/unique_job.tsv".into();
    let write_path3: PathBuf = "./temp/parsed/title-principal/unique_characters.tsv".into();

    let error_write_path1: PathBuf = "./temp/parsed/title-principal/error_tconst.tsv".into();
    let error_write_path2: PathBuf = "./temp/parsed/title-principal/error_nconst.tsv".into();
    let error_write_path3: PathBuf = "./temp/parsed/title-principal/error_profession.tsv".into();

    let mut wtr1 = csv_writer(&write_path1)?;

    let mut unique_job = HashSet::new();
    let mut unique_characters = HashSet::new();

    let mut index: u64 = 1;

    let now = Instant::now();
    while rdr.read_record(&mut raw_record)? {
        let record: TitlePrincipals = match raw_record.deserialize(Some(&headers)) {
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

        let person_id = match ids_person_map.get(record.nconst).copied() {
            Some(id) => id,
            None => {
                unknow_person_nconst.insert(record.nconst.to_string());
                continue;
            }
        };

        let profession_id = match ids_profession_map.get(record.category).copied() {
            Some(id) => id,
            None => {
                unknow_profession_id.insert(record.category.to_string());
                continue;
            }
        };

        let len = record.characters.len();

        // tt0030143	1	nm0071636	actor	\N	["Hal \"Chopper' Donovan, aka Hal Smith"]
        // ^
        // |
        // 29611	1	68337	2	"\N"	"Hal \\"Chopper' Donovan, aka Hal Smith"
        let characters = if record.characters != "\\N" && len >= 4 {
            &record.characters[2..len - 2].replace("\\", "")
        } else {
            record.characters
        };

        wtr1.serialize(Principals {
            movie_id,
            ordering: record.ordering,
            person_id,
            profession_id,
            job: record.job,
            characters,
        })?;

        unique_job.insert(record.job.to_owned());
        unique_characters.insert(characters.to_owned());

        if index % 10000 == 0 {
            wtr1.flush()?;
        }

        index += 1;
    }
    wtr1.flush()?;

    let elapsed_time = now.elapsed();
    println!(
        "Done parsing {} in {} ms.",
        file_path.to_str().unwrap_or("invalid path"),
        elapsed_time.as_millis()
    );

    let mut unique_job_csv = unique_job.into_iter().collect::<Vec<String>>();
    unique_job_csv.insert(0, "uniqueJob".into());
    let mut unique_characters_csv = unique_characters.into_iter().collect::<Vec<String>>();
    unique_characters_csv.insert(0, "uniqueCharacters".into());

    save_as_csv(&write_path2, &unique_job_csv)?;
    save_as_csv(&write_path3, &unique_characters_csv)?;

    let mut file = File::create(error_write_path1)?;
    let v = unknow_movie_tconst.into_iter().collect::<Vec<String>>();
    file.write_all(v.join("\n").as_bytes())?;
    println!(
        "imdb id not found for title_id/tconst : \n{}",
        v.join("\n").as_str()
    );

    let mut file = File::create(error_write_path2)?;
    let v = unknow_person_nconst.into_iter().collect::<Vec<String>>();
    file.write_all(v.join("\n").as_bytes())?;
    println!(
        "imdb id not found for name_id/nconst : \n{}",
        v.join("\n").as_str()
    );

    let mut file = File::create(error_write_path3)?;
    let v = unknow_profession_id.into_iter().collect::<Vec<String>>();
    file.write_all(v.join("\n").as_bytes())?;
    println!(
        "imdb id not found for profession : \n{}",
        v.join("\n").as_str()
    );
    Ok(())
}
