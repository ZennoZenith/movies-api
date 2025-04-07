use std::path::PathBuf;

fn main() {
    println!("Hello, world!");
    let _name_basics_path: PathBuf = "./temp/imdb-tsv/name.basics.tsv".into();
    let _title_akas_path: PathBuf = "./temp/imdb-tsv/title.akas.tsv".into();
    let _title_basics_path: PathBuf = "./temp/imdb-tsv/title.basics.tsv".into();
    let _title_crew_path: PathBuf = "./temp/imdb-tsv/title.crew.tsv".into();
    let _title_episode_path: PathBuf = "./temp/imdb-tsv/title.episode.tsv".into();
    let _title_principals_path: PathBuf = "./temp/imdb-tsv/title.principals.tsv".into();
    let _title_ratings_path: PathBuf = "./temp/imdb-tsv/title.ratings.tsv".into();

    // use movies_api_tsv_parser::models::movies::parse_and_save_imdb_title_basic;
    // parse_and_save_imdb_title_basic(&_title_basics_path).unwrap();

    use movies_api_tsv_parser::models::movies::parse_and_save_imdb_title_akas;
    parse_and_save_imdb_title_akas(&_title_akas_path).unwrap();

    // use movies_api_tsv_parser::models::names::parse_and_save_imdb_name_basic;
    // parse_and_save_imdb_name_basic(&_name_basics_path).unwrap();

    // use movies_api_tsv_parser::models::ratings::parse_and_save_imdb_title_rating;
    // parse_and_save_imdb_title_rating(&_title_ratings_path).unwrap();

    // use movies_api_tsv_parser::models::crews::parse_and_save_imdb_title_crew;
    // parse_and_save_imdb_title_crew(&_title_crew_path).unwrap();

    // use movies_api_tsv_parser::models::episodes::parse_and_save_title_basic;
    // parse_and_save_title_basic(&_title_episode_path).unwrap();

    // use movies_api_tsv_parser::models::principals::parse_and_save_title_principal;
    // parse_and_save_title_principal(&_title_principals_path).unwrap();
}
