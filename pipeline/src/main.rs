use pipeline::movies::metadata::Movie;
use pipeline::movies::ratings::Rating;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut movies: Vec<Movie> = Movie::as_vec()?;
    let ratings_map = Rating::as_map()?;

    //To modify the ratings field you need a mutable reference to each Movie in the vector
    for movie in &mut movies {
        // Use if let for optional handling.
        if let Some(&rating) = ratings_map.get(&movie.id) {
            movie.rating = Some(rating);
        } else {
            movie.rating = None;
        }
    }

    let fv_star_rom= movies
        .iter()
        .filter(|&movie| movie.rating == Some(5.0))
        .filter(|&movie| movie.genres.contains("Romance"))
        .collect();

    Movie::insert_rows(fv_star_rom);
    Ok(())
}
