use std::{
    collections::{BTreeMap, HashSet},
    fs::{File, create_dir_all},
    io::Write,
    path::PathBuf,
    time::Instant,
};

use crate::{
    models::movies::load_movie_id_tconst,
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
pub struct Ratings {
    movie_id: u64,
    /// averageRating – weighted average of all the individual user ratings
    average_rating: f64,
    /// numVotes - number of votes the title has received
    num_votes: u32,
}

pub fn parse_and_save_imdb_title_rating(file_path: &PathBuf) -> Result<(), CsvParseError> {
    create_dir_all("./temp/parsed/title-rating")
        .expect("cannot create dir: ./temp/parsed/title-rating");

    let mut unknow_movie_tconst = HashSet::new();

    let ids_movie = load_movie_id_tconst()?;
    let ids_movie_map: BTreeMap<&str, u64> = ids_movie
        .iter()
        .map(|data| (data.tconst.as_str(), data.movie_id))
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

        match ids_movie_map.get(record.tconst).copied() {
            Some(movie_id) => wtr1.serialize(Ratings {
                movie_id,
                average_rating: record.average_rating,
                num_votes: record.num_votes,
            })?,
            None => {
                unknow_movie_tconst.insert(record.tconst.to_string());
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
    let v = unknow_movie_tconst.into_iter().collect::<Vec<String>>();
    file.write_all(v.join("\n").as_bytes())?;

    println!(
        "imdb id not found for title_id/tconst : \n{}",
        v.join("\n").as_str()
    );
    Ok(())
}
