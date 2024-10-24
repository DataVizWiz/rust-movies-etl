use crate::utils::io::read_csv;
use chrono::NaiveDate;
use postgres::{Client, NoTls};
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
pub struct Movie {
    //Handle empty values with Option enum
    pub adult: String,
    pub belongs_to_collection: Option<String>,
    pub budget: i32,
    pub genres: String,
    pub homepage: Option<String>,
    pub id: i32,
    pub imdb_id: String,
    pub original_language: String,
    pub original_title: String,
    pub overview: String,
    pub popularity: f32,
    pub poster_path: Option<String>,
    pub production_companies: String,
    pub production_countries: String,
    pub release_date: Option<NaiveDate>,
    pub revenue: f32,
    pub runtime: Option<f32>,
    pub spoken_languages: String,
    pub status: String,
    pub tagline: String,
    pub title: String,
    pub video: String,
    pub vote_average: f32,
    pub vote_count: i32,
    pub rating: Option<f32>,
}

impl Movie {
    pub fn as_vec() -> Result<Vec<Movie>, csv::Error> {
        let mut reader = read_csv("data/movies_metadata.csv")?;
        let mut movies: Vec<Movie> = Vec::new();

        for result in reader.deserialize() {
            let movie: Movie = result?;
            movies.push(movie);
        }
        Ok(movies)
    }

    fn db_client() -> Client {
        let conn_string = env::var("DB_CONN_STRING").expect("Key not found");
        Client::connect(&conn_string, NoTls).unwrap()
    }

    pub fn insert_rows(rows: Vec<&Movie>) {
        let mut client = Self::db_client();

        client
            .batch_execute(
                "
        CREATE TABLE IF NOT EXISTS five_star_romance (
            id                  SERIAL PRIMARY KEY,
            title               VARCHAR NOT NULL,
            release_date        DATE,
            budget              INT,
            popularity          REAL,
            revenue             REAL,
            runtime             REAL,
            vote_average        REAL,
            vote_count          INT,
            rating              REAL
        )
        ",
            )
            .unwrap();

        for param in rows {
            client
            .execute(
                "INSERT INTO five_star_romance (
                    title, release_date, budget, popularity, revenue, runtime, vote_average, vote_count, rating
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
                &[
                    &param.title,
                    &param.release_date,
                    &param.budget,
                    &param.popularity,
                    &param.revenue,
                    &param.runtime,
                    &param.vote_average,
                    &param.vote_count,
                    &param.rating,
                ],
            )
            .unwrap();
        }
    }
}
