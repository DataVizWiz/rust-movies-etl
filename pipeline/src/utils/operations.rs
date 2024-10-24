use crate::movies::metadata::Movie;
use crate::utils::io::read_csv;
use csv::Reader;
use std::collections::HashMap;
use std::fs::File;

pub fn get_rec_pos() {
    let mut rdr = read_csv("data/movies_metadata.csv").expect("Error reading file");

    for res in rdr.records() {
        let rec = res.unwrap();
        let pos = rec.position().unwrap();

        println!("{:?} | {:?}", pos, rec);
    }
}

pub fn create_tbl_map() -> Result<HashMap<String, Reader<File>>, std::io::Error> {
    let tbls: [&str; 7] = [
        "credits",
        "keywords",
        "links_small",
        "links",
        "movies_metadata",
        "ratings_small",
        "ratings",
    ];

    let mut map: HashMap<String, Reader<File>> = HashMap::new();

    for name in tbls {
        let path = format!("data/{name}.csv");
        let rdr = read_csv(&path)?;
        map.insert(String::from(name), rdr);
    }
    Ok(map)
}

pub fn loop_vec(movies: Vec<Movie>) {
    /*
    Borrow each element using & to avoid moving them or transferring ownership
    Other methods for looping to know are:
         a. .iter()
         b. .iter_mut()
    */
    for movie in &movies {
        println!("{}: {}", movie.original_title, movie.budget);
    }
}
