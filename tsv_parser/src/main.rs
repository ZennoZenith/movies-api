use std::path::PathBuf;

use movies_api_tsv_parser::models::movies::parse_and_save_imdb_title_basic;

fn main() {
    println!("Hello, world!");
    let _name_basics_path: PathBuf = "./temp/imdb-tsv/name.basics.tsv".into();
    let _title_akas_path: PathBuf = "./temp/imdb-tsv/title.akas.tsv".into();
    let _title_basics_path: PathBuf = "./temp/imdb-tsv/title.basics.tsv".into();
    let _title_crew_path: PathBuf = "./temp/imdb-tsv/title.crew.tsv".into();
    let _title_episode_path: PathBuf = "./temp/imdb-tsv/title.episode.tsv".into();
    let _title_principals_path: PathBuf = "./temp/imdb-tsv/title.principals.tsv".into();
    let _title_ratings_path: PathBuf = "./temp/imdb-tsv/title.ratings.tsv".into();

    parse_and_save_imdb_title_basic(&_title_basics_path).unwrap();
}
