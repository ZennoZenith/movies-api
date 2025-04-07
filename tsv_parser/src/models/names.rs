use std::{
    collections::{BTreeMap, HashSet},
    fs::File,
    io::Write,
    path::PathBuf,
    time::Instant,
};

use crate::{
    models::movies::load_ids_imdb,
    utils::{CsvParseError, csv_reader, csv_writer, save_as_csv},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NameBasic<'a> {
    /// nconst (string) - alphanumeric unique identifier of the name/person
    nconst: &'a str,
    /// primaryName (string)– name by which the person is most often credited
    primary_name: &'a str,
    /// birthYear – in YYYY format
    birth_year: &'a str,
    /// deathYear – in YYYY format if applicable, else '\N'
    death_year: &'a str,
    /// primaryProfession (array of strings)– the top-3 professions of the person
    primary_profession: &'a str,
    /// knownForTitles (array of tconsts) – titles the person is known for
    known_for_titles: &'a str,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImdbName<'a> {
    name_id: u64,
    /// primaryName (string)– name by which the person is most often credited
    primary_name: &'a str,
    /// birthYear – in YYYY format
    birth_year: &'a str,
    /// deathYear – in YYYY format if applicable, else '\N'
    death_year: &'a str,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImdbNameId<'a> {
    pub name_id: u64,
    /// nconst (string) - alphanumeric unique identifier of the name/person
    pub nconst: &'a str,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfessionId {
    pub profession_id: u16,
    pub profession: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NameProfession {
    pub name_id: u64,
    pub profession_id: u16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NameKnowFor {
    pub name_id: u64,
    pub imdb_id: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImdbNameIdSized {
    pub name_id: u64,
    /// nconst (string) - alphanumeric unique identifier of the name/person
    pub nconst: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfessionIdSized {
    pub profession_id: u16,
    pub profession: String,
}

pub fn load_ids_names() -> Result<Vec<ImdbNameIdSized>, CsvParseError> {
    let path: PathBuf = "./temp/parsed/name-basic/ids_name.tsv".into();
    let mut records = Vec::new();

    let mut rdr = csv_reader(&path, false)?;
    for (index, result) in rdr.deserialize::<ImdbNameIdSized>().enumerate() {
        match result {
            Ok(r) => records.push(r),
            Err(e) => println!("index: {}, error: {:?}", index, e),
        }
    }
    Ok(records)
}

pub fn load_ids_professions() -> Result<Vec<ProfessionIdSized>, CsvParseError> {
    let path: PathBuf = "./temp/parsed/name-basic/professions.tsv".into();
    let mut records = Vec::new();

    let mut rdr = csv_reader(&path, false)?;
    for (index, result) in rdr.deserialize::<ProfessionIdSized>().enumerate() {
        match result {
            Ok(r) => records.push(r),
            Err(e) => println!("index: {}, error: {:?}", index, e),
        }
    }
    Ok(records)
}

pub fn parse_and_save_imdb_name_basic(file_path: &PathBuf) -> Result<(), CsvParseError> {
    let ids_imdb = load_ids_imdb()?;
    let mut unknow_imdb_id = HashSet::new();

    let ids_imdb_map: BTreeMap<&str, u64> = ids_imdb
        .iter()
        .map(|data| (data.tconst.as_str(), data.imdb_id))
        .collect();

    let mut rdr = csv_reader(file_path, false)?;
    let mut raw_record = csv::StringRecord::new();
    let mut ids_profession: BTreeMap<String, u16> = BTreeMap::new();
    let mut profession_id = 1_u16;
    let headers = rdr.headers()?.clone();

    println!("Parsing {}", file_path.to_str().unwrap_or("invalid path"));

    let write_path1: PathBuf = "./temp/parsed/name-basic/fixed_name_basic.tsv".into();
    let write_path2: PathBuf = "./temp/parsed/name-basic/names.tsv".into();
    let write_path3: PathBuf = "./temp/parsed/name-basic/ids_name.tsv".into();
    let write_path4: PathBuf = "./temp/parsed/name-basic/name_to_profession.tsv".into();
    let write_path5: PathBuf = "./temp/parsed/name-basic/name_to_imdb_ids.tsv".into();
    let write_path6: PathBuf = "./temp/parsed/name-basic/professions.tsv".into();

    let error_write_path1: PathBuf = "./temp/parsed/name-basic/error.tsv".into();

    let mut wtr1 = csv_writer(&write_path1)?;
    let mut wtr2 = csv_writer(&write_path2)?;
    let mut wtr3 = csv_writer(&write_path3)?;
    let mut wtr4 = csv_writer(&write_path4)?;
    let mut wtr5 = csv_writer(&write_path5)?;

    let mut name_id: u64 = 1;

    ids_profession.insert("self".to_owned(), profession_id);
    profession_id += 1;

    let now = Instant::now();
    while rdr.read_record(&mut raw_record)? {
        let record: NameBasic = match raw_record.deserialize(Some(&headers)) {
            Ok(r) => r,
            Err(e) => {
                println!("index: {}, error: {:?}", name_id, e);
                continue;
            }
        };

        let record_clone = record.clone();
        wtr1.serialize(record)?;

        wtr2.serialize(ImdbName {
            name_id,
            primary_name: record_clone.primary_name,
            birth_year: record_clone.birth_year,
            death_year: record_clone.death_year,
        })?;

        wtr3.serialize(ImdbNameId {
            name_id,
            nconst: record_clone.nconst,
        })?;

        for profession in record_clone.primary_profession.split(",") {
            if let Some(current_profession_id) = ids_profession.get(profession) {
                wtr4.serialize(NameProfession {
                    name_id,
                    profession_id: *current_profession_id,
                })?;
            } else if profession == "\\N" {
                continue;
            } else {
                wtr4.serialize(NameProfession {
                    name_id,
                    profession_id,
                })?;
                let temp = ids_profession.insert(profession.to_string(), profession_id);
                assert_eq!(
                    temp, None,
                    "Logic error, Overwriting value in ids_profession map"
                );
                profession_id += 1;
            }
        }

        for tconst in record_clone.known_for_titles.split(",") {
            if tconst == "\\N" {
                continue;
            }
            match ids_imdb_map.get(tconst).copied() {
                Some(imdb_id) => wtr5.serialize(NameKnowFor { name_id, imdb_id })?,
                None => {
                    unknow_imdb_id.insert(tconst.to_string());
                }
            };
        }

        if name_id % 10000 == 0 {
            wtr1.flush()?;
            wtr2.flush()?;
            wtr3.flush()?;
            wtr4.flush()?;
            wtr5.flush()?;
        }

        name_id += 1;
    }
    wtr1.flush()?;
    wtr2.flush()?;
    wtr3.flush()?;
    wtr4.flush()?;
    wtr5.flush()?;

    let mut ids_profession_sorted = ids_profession
        .into_iter()
        .map(|(profession, profession_id)| ProfessionId {
            profession_id,
            profession,
        })
        .collect::<Vec<ProfessionId>>();

    ids_profession_sorted.sort_by_key(|key| key.profession_id);
    save_as_csv(&write_path6, &ids_profession_sorted)?;

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
    Ok(())
}
