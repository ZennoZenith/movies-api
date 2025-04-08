ALTER TABLE genres
ADD PRIMARY KEY (id),
ADD UNIQUE (genre);

ALTER TABLE movies
ADD PRIMARY KEY (id),
ADD CHECK (start_year > 0),
ADD CHECK (end_year > 0),
ADD CHECK (end_year >= start_year);

ALTER TABLE movie_genres
ADD PRIMARY KEY (movie_id, genre_id),
ADD FOREIGN KEY (movie_id)
    REFERENCES movies (id)
    ON DELETE RESTRICT,
ADD FOREIGN KEY (genre_id)
    REFERENCES genres (id)
    ON DELETE RESTRICT;

ALTER TABLE movie_tconst
ADD UNIQUE (movie_id),
ADD UNIQUE (tconst),
ADD FOREIGN KEY (movie_id)
    REFERENCES movies (id)
    ON DELETE RESTRICT;

ALTER TABLE languages
ADD PRIMARY KEY (id),
ADD UNIQUE (imdb_language_code);

ALTER TABLE countries
ADD PRIMARY KEY (id),
ADD UNIQUE (imdb_country_code);

ALTER TABLE movie_akas
ADD PRIMARY KEY (movie_id, ordering),
ADD CHECK (ordering > 0),
ADD FOREIGN KEY (movie_id)
    REFERENCES movies (id)
    ON DELETE RESTRICT,
ADD FOREIGN KEY (country_id)
    REFERENCES countries (id)
    ON DELETE RESTRICT,
ADD FOREIGN KEY (language_id)
    REFERENCES languages (id)
    ON DELETE RESTRICT;

ALTER TABLE professions
ADD PRIMARY KEY (id),
ADD UNIQUE (profession);

ALTER TABLE persons
ADD PRIMARY KEY (id),
ADD CHECK (birth_year > 0),
ADD CHECK (death_year > 0);
-- in database some death_year is less than birth_year
-- ADD CHECK (death_year >= birth_year);

ALTER TABLE person_nconst
ADD UNIQUE (person_id),
ADD UNIQUE (nconst),
ADD FOREIGN KEY (person_id)
    REFERENCES persons (id)
    ON DELETE RESTRICT;

ALTER TABLE person_movies
ADD PRIMARY KEY (person_id, movie_id),
ADD FOREIGN KEY (person_id)
    REFERENCES persons (id)
    ON DELETE RESTRICT,
ADD FOREIGN KEY (movie_id)
    REFERENCES movies (id)
    ON DELETE RESTRICT;

ALTER TABLE person_professions
ADD PRIMARY KEY (person_id, profession_id),
ADD FOREIGN KEY (person_id)
    REFERENCES persons (id)
    ON DELETE RESTRICT,
ADD FOREIGN KEY (profession_id)
    REFERENCES professions (id)
    ON DELETE RESTRICT;

ALTER TABLE ratings  
ADD PRIMARY KEY (movie_id),
ADD CHECK (average_rating >= 0),
ADD CHECK (num_votes >= 0),
ADD FOREIGN KEY (movie_id)
    REFERENCES movies (id)
    ON DELETE RESTRICT;


ALTER TABLE shows
ADD PRIMARY KEY (show_id, episode_id),
ADD CHECK (season_number >= 0),
ADD CHECK (episode_number >= 0),
ADD FOREIGN KEY (show_id)
    REFERENCES movies (id)
    ON DELETE RESTRICT,
ADD FOREIGN KEY (episode_id)
    REFERENCES movies (id)
    ON DELETE RESTRICT;

ALTER TABLE movie_directors
ADD PRIMARY KEY (movie_id, person_id),
ADD FOREIGN KEY (movie_id)
    REFERENCES movies (id)
    ON DELETE RESTRICT,
ADD FOREIGN KEY (person_id)
    REFERENCES persons (id)
    ON DELETE RESTRICT;

ALTER TABLE movie_writers
ADD PRIMARY KEY (movie_id, person_id),
ADD FOREIGN KEY (movie_id)
    REFERENCES movies (id)
    ON DELETE RESTRICT,
ADD FOREIGN KEY (person_id)
    REFERENCES persons (id)
    ON DELETE RESTRICT;

ALTER TABLE principals
ADD UNIQUE (movie_id, ordering),
-- ADD UNIQUE (movie_id, person_id),
-- (90748, 284231) is not unique, 90748	-> tt0092811, 284231 -> nm0300272
ADD CHECK (ordering > 0),
ADD FOREIGN KEY (movie_id)
    REFERENCES movies (id)
    ON DELETE RESTRICT,
ADD FOREIGN KEY (person_id)
    REFERENCES persons (id)
    ON DELETE RESTRICT,
ADD FOREIGN KEY (profession_id)
    REFERENCES professions (id)
    ON DELETE RESTRICT;
