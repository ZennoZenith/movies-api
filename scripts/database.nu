cargo install sqlx-cli

mkdir sql-csv/

cp temp/parsed/title-basic/movies.tsv sql-csv/
cp temp/parsed/title-basic/genres.tsv sql-csv/
cp temp/parsed/title-basic/movie_id_to_tconst.tsv sql-csv/
cp temp/parsed/title-basic/movie_id_to_genre_id.tsv sql-csv/

cp temp/codes/languages_imdb.tsv sql-csv/
cp temp/codes/countries_imdb.tsv sql-csv/

cp temp/parsed/title-akas/movie_akas.tsv sql-csv/

cp temp/parsed/name-basic/persons.tsv sql-csv/
cp temp/parsed/name-basic/person_id_to_nconst.tsv sql-csv/
cp temp/parsed/name-basic/person_to_profession.tsv sql-csv/
cp temp/parsed/name-basic/person_to_movie_ids.tsv sql-csv/
cp temp/parsed/name-basic/professions.tsv sql-csv/

cp temp/parsed/title-crew/crew_writer.tsv sql-csv/
cp temp/parsed/title-crew/crew_director.tsv sql-csv/

cp temp/parsed/title-rating/ratings.tsv sql-csv/

cp temp/parsed/title-episodes/shows.tsv sql-csv/

cp temp/parsed/title-principal/principals.tsv sql-csv/

sqlx migrate run


cp temp/parsed/title-basic/genres.tsv sql-csv/
cp temp/parsed/title-basic/movies.tsv sql-csv/
cp temp/parsed/title-basic/movie_id_to_tconst.tsv sql-csv/
cp temp/parsed/title-basic/movie_id_to_genre_id.tsv sql-csv/


let step_01_create_tables = open --raw ./sql/step-01-create-tables.sql
let step_02_add_constraints = open --raw ./sql/step-02-add-constraints.sql

let options = "
  FORMAT CSV,
  DELIMITER '\t',
  HEADER true,
  NULL '\\N',
  ESCAPE '\\',
  QUOTE '\"',
  FORCE_NULL *
"


let database_info = [
  [ local_file_path docker_file_path table_name];
  # [ './sql-csv/genres.tsv' '/csvs/genres.tsv' genres]
  # [ './sql-csv/movies.tsv' '/csvs/movies.tsv' movies]
  # [ './sql-csv/movie_id_to_genre_id.tsv' '/csvs/movie_id_to_genre_id.tsv' movie_genres]
  # [ './sql-csv/movie_id_to_tconst.tsv' '/csvs/movie_id_to_tconst.tsv' movie_tconst]
  # [ './sql-csv/languages_imdb.tsv' '/csvs/languages_imdb.tsv' languages]
  # [ './sql-csv/countries_imdb.tsv' '/csvs/countries_imdb.tsv' countries]
  # [ './sql-csv/movie_akas.tsv' '/csvs/movie_akas.tsv' movie_akas]
  # [ './sql-csv/professions.tsv' '/csvs/professions.tsv' professions]
  # [ './sql-csv/persons.tsv' '/csvs/persons.tsv' persons]
  # [ './sql-csv/person_id_to_nconst.tsv' '/csvs/person_id_to_nconst.tsv' person_nconst]
  # [ './sql-csv/person_to_movie_ids.tsv' '/csvs/person_to_movie_ids.tsv' person_movies]
  # [ './sql-csv/person_to_profession.tsv' '/csvs/person_to_profession.tsv' person_professions]
  # [ './sql-csv/ratings.tsv' '/csvs/ratings.tsv' ratings]
  # [ './sql-csv/shows.tsv' '/csvs/shows.tsv' shows]
  # [ './sql-csv/crew_director.tsv' '/csvs/crew_director.tsv' movie_directors]
  # [ './sql-csv/crew_writer.tsv' '/csvs/crew_writer.tsv' movie_writers]
  [ './sql-csv/principals.tsv' '/csvs/principals.tsv' principals]
]

let copy_commands = $database_info | each { |val|
  let local_file_path = $val.local_file_path
  let docker_file_path = $val.docker_file_path
  let table_name = $val.table_name
  let header = open --raw $local_file_path | head -n 1 | str replace --all "\t" ", "

  let copy_command = [
    "COPY" 
    $table_name
    "("
    $header
    ") FROM"
    $"'($docker_file_path)'"
    "WITH ("
    $options
    ");"
  ] | str join " "

  return {
    local_file_path: $local_file_path,
    docker_file_path: $docker_file_path,
    table_name: $table_name,
    header: $header,
    copy_command: $copy_command
  }
}

# let movies = [
#   "COPY movies(id, title_type, primary_title, original_title, is_adult, start_year, end_year, runtime_minutes) FROM '/csvs/movies.tsv' WITH ("
#   $options
#   ");"
# ] | str join " "


################################### DATABASE ##################################

let port = "9969"
let host = "zenith"
let user = "postgres"
let database_name = "imdb_movies_db"
let dump_file_name = "dump"

$env.PGPASSWORD = "password"

psql -h $host -p $port -U $user -d $database_name -c $step_01_create_tables

$copy_commands | each { |val|
  print $"Copying from ($val.docker_file_path) to table ($val.table_name)"
  let time_elasp = timeit {
    psql -h $host -p $port -U $user -d $database_name -c $val.copy_command
  }
  print $"Done ($val.table_name) in ($time_elasp)"
  $"Done ($val.table_name) in ($time_elasp)"
}
 
psql -h $host -p $port -U $user -d $database_name -c $step_02_add_constraints

# pg_dump -h $host -p $port -U $user --schema-only $database_name | save -f $'($dump_file_name)_schema_only.sql'
