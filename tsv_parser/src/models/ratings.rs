use std::{
    collections::{BTreeMap, HashSet},
    fs::File,
    io::Write,
    path::PathBuf,
    time::Instant,
};

use crate::{
    models::movies::load_ids_imdb,
    utils::{CsvParseError, csv_reader, csv_writer},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TitleRating<'a> {
    /// tconst (string) - alphanumeric unique identifier of the title
    tconst: &'a str,
    /// averageRating – weighted average of all the individual user ratings
    average_rating: f64,
    /// numVotes - number of votes the title has received
    num_votes: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Ratings {
    imdb_id: u64,
    /// averageRating – weighted average of all the individual user ratings
    average_rating: f64,
    /// numVotes - number of votes the title has received
    num_votes: u32,
}

pub fn parse_and_save_imdb_title_rating(file_path: &PathBuf) -> Result<(), CsvParseError> {
    let mut unknow_imdb_id = HashSet::new();

    let ids_imdb = load_ids_imdb()?;
    let ids_imdb_map: BTreeMap<&str, u64> = ids_imdb
        .iter()
        .map(|data| (data.tconst.as_str(), data.imdb_id))
        .collect();

    let mut rdr = csv_reader(file_path, false)?;
    let mut raw_record = csv::StringRecord::new();
    let headers = rdr.headers()?.clone();

    let write_path1: PathBuf = "./temp/parsed/title-rating/ratings.tsv".into();

    let error_write_path1: PathBuf = "./temp/parsed/title-rating/error.tsv".into();

    let mut wtr1 = csv_writer(&write_path1)?;

    let mut index: u64 = 1;
    let now = Instant::now();
    while rdr.read_record(&mut raw_record)? {
        let record: TitleRating = match raw_record.deserialize(Some(&headers)) {
            Ok(r) => r,
            Err(e) => {
                println!("index: {}, error: {:?}", index, e);
                continue;
            }
        };

        match ids_imdb_map.get(record.tconst).copied() {
            Some(imdb_id) => wtr1.serialize(Ratings {
                imdb_id,
                average_rating: record.average_rating,
                num_votes: record.num_votes,
            })?,
            None => {
                unknow_imdb_id.insert(record.tconst.to_string());
            }
        };

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

    let mut file = File::create(error_write_path1)?;
    let v = unknow_imdb_id.into_iter().collect::<Vec<String>>();
    file.write_all(v.join("\n").as_bytes())?;

    println!(
        "imdb id not found for title_id/tconst : \n{}",
        v.join("\n").as_str()
    );
    Ok(())
}
