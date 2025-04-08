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
pub struct TitleEpisode<'a> {
    ///tconst (string) - alphanumeric identifier of episode
    tconst: &'a str,
    ///parentTconst (string) - alphanumeric identifier of the parent TV Series
    parent_tconst: &'a str,
    ///seasonNumber (integer) – season number the episode belongs to
    season_number: &'a str,
    ///episodeNumber (integer) – episode number of the tconst in the TV series
    episode_number: &'a str,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Shows<'a> {
    pub show_id: u64,
    pub episode_id: u64,
    season_number: &'a str,
    episode_number: &'a str,
}

pub fn parse_and_save_title_episode(file_path: &PathBuf) -> Result<(), CsvParseError> {
    create_dir_all("./temp/parsed/title-episodes")
        .expect("cannot create dir: ./temp/parsed/title-episodes");

    let ids_movie = load_movie_id_tconst()?;
    let mut unknow_movie_tconst = HashSet::new();

    let ids_imdb_map: BTreeMap<&str, u64> = ids_movie
        .iter()
        .map(|data| (data.tconst.as_str(), data.movie_id))
        .collect();

    let mut rdr = csv_reader(file_path, false)?;
    let mut raw_record = csv::StringRecord::new();
    let headers = rdr.headers()?.clone();

    println!("Parsing {}", file_path.to_str().unwrap_or("invalid path"));

    let write_path1: PathBuf = "./temp/parsed/title-episodes/shows.tsv".into();

    let error_write_path1: PathBuf = "./temp/parsed/title-episodes/error.tsv".into();

    let mut wtr1 = csv_writer(&write_path1)?;

    let mut index: u64 = 1;

    let now = Instant::now();
    while rdr.read_record(&mut raw_record)? {
        let record: TitleEpisode = match raw_record.deserialize(Some(&headers)) {
            Ok(r) => r,
            Err(e) => {
                println!("index: {}, error: {:?}", index, e);
                continue;
            }
        };

        let show_id = match ids_imdb_map.get(record.tconst).copied() {
            Some(id) => id,
            None => {
                unknow_movie_tconst.insert(record.tconst.to_string());
                continue;
            }
        };
        let episode_id = match ids_imdb_map.get(record.tconst).copied() {
            Some(id) => id,
            None => {
                unknow_movie_tconst.insert(record.tconst.to_string());
                continue;
            }
        };

        wtr1.serialize(Shows {
            show_id,
            episode_id,
            season_number: record.season_number,
            episode_number: record.episode_number,
        })?;

        if index % 10000 == 0 {
            wtr1.flush()?;
        }

        index += 1;

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
