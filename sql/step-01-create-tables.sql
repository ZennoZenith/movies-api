CREATE TYPE title_types AS ENUM ( 
    'titleType',
    'short',
    'tvEpisode',
    'tvSpecial',
    'video',
    'movie',
    'tvShort',
    'tvMovie',
    'tvSeries',
    'tvPilot',
    'videoGame',
    'tvMiniSeries'
);

CREATE TABLE genres (
    id smallserial NOT NULL,
    genre text NOT NULL
);

CREATE TABLE movies (
    id bigserial NOT NULL,
    title_type title_types NOT NULL,
    primary_title text NOT NULL,
    original_title text NOT NULL,
    is_adult bool NOT NULL DEFAULT false,
    start_year int2 NULL,
    end_year int2 NULL,
    runtime_minutes int4 NOT NULL DEFAULT 0
);

CREATE TABLE movie_genres (
    movie_id int8 NOT NULL,
    genre_id int2 NOT NULL
);

CREATE TABLE movie_tconst (
    movie_id int8 NOT NULL,
    tconst varchar(12) NOT NULL
);

CREATE TABLE languages (
    id smallserial NOT NULL,
    iso_639_1 text DEFAULT NULL,
    iso_639_2 text DEFAULT NULL,
    language_name_iso_639_2 text NOT NULL,
    imdb_language_code text DEFAULT NULL
);

CREATE TABLE countries (
    id smallserial NOT NULL,
    name text NOT NULL,
    alpha_3 text DEFAULT NULL,
    country_code text DEFAULT NULL,
    iso_3166_2 text DEFAULT NULL,
    region text DEFAULT NULL,
    sub_region text DEFAULT NULL,
    intermediate_region text DEFAULT NULL,
    imdb_country_code text NOT NULL
);

CREATE TABLE movie_akas (
    movie_id int8 NOT NULL,
    ordering int2 NOT NULL,
    title text NOT NULL,
    country_id int2 DEFAULT NULL,
    language_id int2 DEFAULT NULL,
    types text DEFAULT NULL,
    attributes text DEFAULT NULL,
    is_original_title bool NOT NULL DEFAULT false
);

CREATE TABLE professions (
    id smallserial NOT NULL,
    profession text NOT NULL
);

CREATE TABLE persons (
    id bigserial NOT NULL,
    primary_name text NOT NULL,
    birth_year int2 NULL,
    death_year int2 NULL,
    runtime_minutes int2 NOT NULL DEFAULT 0
);

CREATE TABLE person_nconst (
    person_id int8 NOT NULL,
    nconst varchar(12) NOT NULL
);

CREATE TABLE person_movies (
    person_id int8 NOT NULL,
    movie_id int8 NOT NULL
);

CREATE TABLE person_professions (
    person_id int8 NOT NULL,
    profession_id int2 NOT NULL
);

CREATE TABLE ratings (
    movie_id int8 NOT NULL,
    average_rating real NOT NULL,
    num_votes int4 NOT NULL
);

CREATE TABLE shows (
    show_id int8 NOT NULL,
    episode_id int8 NOT NULL,
    season_number int2 NULL,
    episode_number int4 NULL
);

CREATE TABLE movie_directors (
    movie_id int8 NOT NULL,
    person_id int8 NOT NULL
);

CREATE TABLE movie_writers (
    movie_id int8 NOT NULL,
    person_id int8 NOT NULL
);

CREATE TABLE principals (
    movie_id int8 NOT NULL,
    ordering int2 NOT NULL,
    person_id int8 NOT NULL,
    profession_id int2 NOT NULL,
    job text DEFAULT NULL,
    characters text DEFAULT NULL
);
