use crate::utils::io::read_csv;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Rating {
    #[serde(rename(deserialize = "movieId"))]
    movie_id: i32,
    rating: f32,
}

impl Rating {
    pub fn as_map() -> Result<HashMap<i32, f32>, csv::Error> {
        let mut reader = read_csv("data/ratings_small.csv")?;

        let mut map: HashMap<i32, f32> = HashMap::new();

        for result in reader.deserialize() {
            let rating: Rating = result?;
            map.insert(rating.movie_id, rating.rating);
        }
        Ok(map)
    }
}
