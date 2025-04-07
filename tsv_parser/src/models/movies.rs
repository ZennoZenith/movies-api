use std::{
    collections::{BTreeMap, HashSet},
    fs::create_dir_all,
    path::PathBuf,
    time::Instant,
};

use crate::{
    models::misc::{load_country_code, load_language_code},
    utils::{CsvParseError, csv_reader, csv_writer, save_as_csv},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TitleBasic<'a> {
    /// tconst (string) - alphanumeric unique identifier of the title
    pub tconst: &'a str,
    /// titleType (&'a str) – the type/format of the title (e.g. movie, short, tvseries, tvepisode, video, etc)
    pub title_type: &'a str,
    /// primaryTitle (&'a str) – the more popular title / the title used by the filmmakers on promotional materials at the point of release
    pub primary_title: &'a str,
    /// originalTitle (&'a str) - original title, in the original language
    pub original_title: &'a str,
    /// isAdult (boolean) - 0: non-adult title; 1: adult title
    pub is_adult: &'a str,
    /// startYear (YYYY) – represents the release year of a title. In the case of TV Series, it is the series start year
    pub start_year: &'a str,
    /// endYear (YYYY) – TV Series end year. '\N' for all other title types
    pub end_year: &'a str,
    /// runtimeMinutes – primary runtime of the title, in minutes
    pub runtime_minutes: &'a str,
    /// genres (&'a str array) – includes up to three genres associated with the title
    pub genres: &'a str,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Movie<'a> {
    pub id: u64,
    /// titleType (&'a str ) – the type/format of the title (e.g. movie, short, tvseries, tvepisode, video, etc)
    pub title_type: &'a str,
    /// primaryTitle (&'a str ) – the more popular title / the title used by the filmmakers on promotional materials at the point of release
    pub primary_title: &'a str,
    /// originalTitle (&'a str ) - original title, in the original language
    pub original_title: &'a str,
    /// isAdult (boolean) - 0: non-adult title; 1: adult title
    pub is_adult: bool,
    /// startYear (YYYY) – represents the release year of a title. In the case of TV Series, it is the series start year
    pub start_year: &'a str,
    /// endYear (YYYY) – TV Series end year. '\N' for all other title types
    pub end_year: &'a str,
    /// runtimeMinutes – primary runtime of the title, in minutes
    pub runtime_minutes: u16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GenreSized {
    pub id: u16,
    pub genre: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MovieTconst<'a> {
    pub movie_id: u64,
    /// tconst (string) - alphanumeric unique identifier of the title
    pub tconst: &'a str,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MovieGenre {
    pub movie_id: u64,
    pub genre_id: u16,
}

pub fn parse_and_save_imdb_title_basic(file_path: &PathBuf) -> Result<(), CsvParseError> {
    create_dir_all("./temp/parsed/title-basic")
        .expect("cannot create dir: ./temp/parsed/title-basic");

    let mut rdr = csv_reader(file_path, false)?;
    let mut unique_title_type = HashSet::new();
    let mut raw_record = csv::StringRecord::new();
    let mut ids_genre: BTreeMap<String, u16> = BTreeMap::new();
    let mut genre_id = 1_u16;
    let mut movie_id: u64 = 1;
    let headers = rdr.headers()?.clone();

    println!("Parsing {}", file_path.to_str().unwrap_or("invalid path"));

    let write_path1: PathBuf = "./temp/parsed/title-basic/fixed_imdb_title_basic.tsv".into();
    let write_path2: PathBuf = "./temp/parsed/title-basic/movies.tsv".into();
    let write_path3: PathBuf = "./temp/parsed/title-basic/movie_id_to_tconst.tsv".into();
    let write_path4: PathBuf = "./temp/parsed/title-basic/movie_id_to_genre_id.tsv".into();
    let write_path5: PathBuf = "./temp/parsed/title-basic/unique_title_type.tsv".into();
    let write_path6: PathBuf = "./temp/parsed/title-basic/genres.tsv".into();

    let mut wtr1 = csv_writer(&write_path1)?;
    let mut wtr2 = csv_writer(&write_path2)?;
    let mut wtr3 = csv_writer(&write_path3)?;
    let mut wtr4 = csv_writer(&write_path4)?;

    let now = Instant::now();
    while rdr.read_record(&mut raw_record)? {
        let record: TitleBasic = match raw_record.deserialize(Some(&headers)) {
            Ok(r) => r,
            Err(e) => {
                println!("index: {}, error: {:?}", movie_id, e);
                continue;
            }
        };
        let record_clone = record.clone();
        unique_title_type.insert(record_clone.title_type.to_string());
        wtr1.serialize(record)?;

        wtr2.serialize(Movie {
            id: movie_id,
            title_type: record_clone.title_type,
            primary_title: record_clone.primary_title,
            original_title: record_clone.original_title,
            is_adult: record_clone.is_adult == "1",
            start_year: record_clone.start_year,
            end_year: record_clone.end_year,
            runtime_minutes: record_clone.runtime_minutes.parse().unwrap_or_default(),
        })?;

        wtr3.serialize(MovieTconst {
            movie_id,
            tconst: record_clone.tconst,
        })?;

        for genre in record_clone.genres.split(",") {
            if let Some(current_genre_id) = ids_genre.get(genre) {
                wtr4.serialize(MovieGenre {
                    movie_id,
                    genre_id: *current_genre_id,
                })?;
            } else if genre == "\\N" {
                continue;
            } else {
                wtr4.serialize(MovieGenre { movie_id, genre_id })?;
                let temp = ids_genre.insert(genre.to_string(), genre_id);
                assert_eq!(
                    temp, None,
                    "Logic error, Overwriting value in ids_genre map"
                );
                genre_id += 1;
            }
        }

        if movie_id % 10000 == 0 {
            wtr1.flush()?;
            wtr2.flush()?;
            wtr3.flush()?;
            wtr4.flush()?;
        }

        movie_id += 1;
    }
    wtr1.flush()?;
    wtr2.flush()?;
    wtr3.flush()?;
    wtr4.flush()?;

    let mut unique_title_type_csv: Vec<String> = unique_title_type.into_iter().collect();
    unique_title_type_csv.insert(0, "title_type".into());
    save_as_csv(&write_path5, &unique_title_type_csv)?;

    let mut ids_genre_sorted = ids_genre
        .into_iter()
        .map(|(genre, genre_id)| GenreSized {
            id: genre_id,
            genre,
        })
        .collect::<Vec<GenreSized>>();

    ids_genre_sorted.sort_by_key(|key| key.id);
    save_as_csv(&write_path6, &ids_genre_sorted)?;

    let elapsed_time = now.elapsed();
    println!(
        "Done parsing {} in {} ms.",
        file_path.to_str().unwrap_or("invalid path"),
        elapsed_time.as_millis()
    );

    Ok(())
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MovieTconstSized {
    pub movie_id: u64,
    pub tconst: String,
}

pub fn load_movie_id_tconst() -> Result<Vec<MovieTconstSized>, CsvParseError> {
    let path: PathBuf = "./temp/parsed/title-basic/movie_id_to_tconst.tsv".into();
    let mut records = Vec::new();

    let mut rdr = csv_reader(&path, false)?;
    for (index, result) in rdr.deserialize::<MovieTconstSized>().enumerate() {
        match result {
            Ok(r) => records.push(r),
            Err(e) => println!("index: {}, error: {:?}", index, e),
        }
    }
    Ok(records)
}

// https://help.imdb.com/article/contribution/other-submission-guides/country-codes/G99K4LFRMSC37DCN?ref_=helpart_nav_6#
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TitleAkas<'a> {
    /// titleId (string) - a tconst, an alphanumeric unique identifier of the title (i.e imdb id: eg:tt0000001)
    title_id: &'a str,
    /// ordering (integer) – a number to uniquely identify rows for a given titleId
    ordering: u16,
    /// title (&'a str ) – the localized title
    title: &'a str,
    /// region (&'a str ) - the region for this version of the title
    region: &'a str,
    /// language (&'a str ) - the language of the title
    language: &'a str,
    /// types (array) - Enumerated set of attributes for this alternative title. One or more of the following: "alternative", "dvd", "festival", "tv", "video", "working", "original", "imdbDisplay". New values may be added in the future without warning
    types: &'a str,
    /// attributes (array) - Additional terms to describe this alternative title, not enumerated
    attributes: &'a str,
    /// isOriginalTitle (boolean) – 0: not original title; 1: original title
    is_original_title: &'a str,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MovieAkas<'a> {
    movie_id: u64,
    /// ordering (integer) – a number to uniquely identify rows for a given titleId
    ordering: u16,
    /// title (&'a str) – the localized title
    title: &'a str,
    country_id: &'a str,
    language_id: &'a str,
    /// types (array) - Enumerated set of attributes for this alternative title. One or more of the following: "alternative", "dvd", "festival", "tv", "video", "working", "original", "imdbDisplay". New values may be added in the future without warning
    types: &'a str,
    /// attributes (array) - Additional terms to describe this alternative title, not enumerated
    attributes: &'a str,
    is_original_title: bool,
}

pub fn parse_and_save_imdb_title_akas(file_path: &PathBuf) -> Result<(), CsvParseError> {
    create_dir_all("./temp/parsed/title-akas")
        .expect("cannot create dir: ./temp/parsed/title-akas");

    let movie_id_tconst = load_movie_id_tconst()?;
    let language_id_code = load_language_code()?;
    let country_id_code = load_country_code()?;

    let mut unique_region = HashSet::new();
    let mut unique_language = HashSet::new();
    let mut unique_types = HashSet::new();
    let mut unique_attributes = HashSet::new();

    let ids_tconst_map: BTreeMap<&str, u64> = movie_id_tconst
        .iter()
        .map(|data| (data.tconst.as_str(), data.movie_id))
        .collect();

    let ids_language_map: BTreeMap<&str, u16> = language_id_code
        .iter()
        .map(|data| (data.imdb_language_code.as_str(), data.id))
        .collect();

    let ids_country_map: BTreeMap<&str, u16> = country_id_code
        .iter()
        .map(|data| (data.imdb_country_code.as_str(), data.id))
        .collect();

    let mut rdr = csv_reader(file_path, false)?;

    let mut raw_record = csv::StringRecord::new();
    let headers = rdr.headers()?.clone();

    println!("Parsing {}", file_path.to_str().unwrap_or("invalid path"));
    let now = Instant::now();

    let write_path1: PathBuf = "./temp/parsed/title-akas/fixed_imdb_title_akas.tsv".into();
    let write_path2: PathBuf = "./temp/parsed/title-akas/movie_akas.tsv".into();
    let write_path3: PathBuf = "./temp/parsed/title-akas/unique_region.tsv".into();
    let write_path4: PathBuf = "./temp/parsed/title-akas/unique_language.tsv".into();
    let write_path5: PathBuf = "./temp/parsed/title-akas/unique_types.tsv".into();
    let write_path6: PathBuf = "./temp/parsed/title-akas/unique_attributes.tsv".into();

    let mut wtr1 = csv_writer(&write_path1)?;
    let mut wtr2 = csv_writer(&write_path2)?;

    let mut index = 1_u32;

    while rdr.read_record(&mut raw_record)? {
        let record: TitleAkas = match raw_record.deserialize(Some(&headers)) {
            Ok(r) => r,
            Err(e) => {
                println!("index: {}, error: {:?}", index, e);
                continue;
            }
        };
        let record_clone = record.clone();

        wtr1.serialize(record)?;

        wtr2.serialize(MovieAkas {
            movie_id: *ids_tconst_map
                .get(record_clone.title_id)
                .unwrap_or_else(|| {
                    panic!(
                        "imdb id not found for title_id/tconst : {}",
                        record_clone.title_id
                    )
                }),
            ordering: record_clone.ordering,
            title: record_clone.title,
            country_id: &ids_country_map
                .get(record_clone.region)
                .map(|v| v.to_string())
                .unwrap_or("\\N".into()),
            language_id: &ids_language_map
                .get(record_clone.language)
                .map(|v| v.to_string())
                .unwrap_or("\\N".into()),
            types: record_clone
                .types
                .split('\u{0002}')
                .collect::<Vec<&str>>()
                .join(",")
                .as_str(),
            attributes: record_clone
                .attributes
                .split('\u{0002}')
                .collect::<Vec<&str>>()
                .join(",")
                .as_str(),
            is_original_title: record_clone.is_original_title == "1",
        })?;

        unique_region.insert(record_clone.region.to_string());
        unique_language.insert(record_clone.language.to_string());
        unique_types.insert(record_clone.types.to_string());
        unique_attributes.insert(record_clone.attributes.to_string());

        if index % 10000 == 0 {
            wtr1.flush()?;
            wtr2.flush()?;
        }

        index += 1;
    }
    wtr1.flush()?;
    wtr2.flush()?;

    let mut unique_region_csv = unique_region.into_iter().collect::<Vec<String>>();
    unique_region_csv.insert(0, "uniqueRegion".into());
    let mut unique_language_csv = unique_language.into_iter().collect::<Vec<String>>();
    unique_language_csv.insert(0, "uniqueLanguage".into());
    let mut unique_types_csv = unique_types
        .into_iter()
        .map(|v| v.split('\u{0002}').collect::<Vec<&str>>().join(","))
        .collect::<Vec<String>>();
    unique_types_csv.insert(0, "uniqueTypes".into());
    let mut unique_attributes_csv = unique_attributes
        .into_iter()
        .map(|v| v.split('\u{0002}').collect::<Vec<&str>>().join(","))
        .collect::<Vec<String>>();
    unique_attributes_csv.insert(0, "uniqueAttributes".into());

    save_as_csv(&write_path3, &unique_region_csv)?;
    save_as_csv(&write_path4, &unique_language_csv)?;
    save_as_csv(&write_path5, &unique_types_csv)?;
    save_as_csv(&write_path6, &unique_attributes_csv)?;

    let elapsed_time = now.elapsed();
    println!(
        "Done parsing {} in {} ms.",
        file_path.to_str().unwrap_or("invalid path"),
        elapsed_time.as_millis()
    );

    Ok(())
}
