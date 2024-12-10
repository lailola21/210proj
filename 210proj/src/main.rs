use csv::Writer;
use csv::ReaderBuilder;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Movie {
    id: String,
    title: String,
    genres: String,
    release_date: String,
}

fn main() {
  
    let input_file_path = "tmdb_5000_movies.csv";

    
    let output_file_path = "genre_popularity_over_time.csv";

    
    let genre_trends = compute_genre_popularity(input_file_path);

    
    let summary_stats = compute_summary_statistics(&genre_trends);
    print_summary_statistics(&summary_stats);

    
    write_genre_trends_to_csv(&genre_trends, output_file_path);

    println!("Genre popularity trends written to: {}", output_file_path);
}


fn compute_genre_popularity(file_path: &str) -> HashMap<i32, HashMap<String, usize>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)
        .expect("Failed to open the file");

    let mut genre_trends: HashMap<i32, HashMap<String, usize>> = HashMap::new();

    for result in reader.deserialize() {
        let movie: Movie = match result {
            Ok(movie) => movie,
            Err(err) => {
                eprintln!("Skipping malformed row: {}", err);
                continue;
            } 
        };

        if let Some(year) = parse_release_year(&movie.release_date) {
            let genres = parse_genres(&movie.genres);
            for genre in genres {
                let entry = genre_trends
                    .entry(year)
                    .or_insert_with(HashMap::new)
                    .entry(genre)
                    .or_insert(0);
                *entry += 1;
            }
        }
    }

    genre_trends
}


fn parse_release_year(release_date: &str) -> Option<i32> {
    if release_date.is_empty() {
        return None;
    }
    release_date.split('-').next()?.parse().ok()
}

fn parse_genres(genres_json: &str) -> Vec<String> {
    let genres: Vec<Value> = match serde_json::from_str(genres_json) {
        Ok(genres) => genres,
        Err(err) => {
            eprintln!("Skipping invalid JSON in genres: {}", err);
            return vec![];
        }
    };

    genres
        .iter()
        .filter_map(|entry| entry.get("name").and_then(|name| name.as_str()).map(String::from))
        .collect()
}


fn write_genre_trends_to_csv(
    genre_trends: &HashMap<i32, HashMap<String, usize>>,
    file_path: &str,
) {
    let mut writer = Writer::from_path(file_path).expect("Failed to create the output file");

    // Write headers
    writer
        .write_record(&["Year", "Genre", "Count"])
        .expect("Failed to write headers");

    // Write data
    for (year, genres) in genre_trends {
        for (genre, count) in genres {
            writer
                .write_record(&[year.to_string(), genre.clone(), count.to_string()])
                .expect("Failed to write a record");
        }
    }

    writer.flush().expect("Failed to flush writer");
}


fn compute_summary_statistics(
    genre_trends: &HashMap<i32, HashMap<String, usize>>,
) -> HashMap<String, (usize, i32, i32)> {
    let mut stats: HashMap<String, (usize, i32, i32)> = HashMap::new();

    for (year, genres) in genre_trends {
        for (genre, count) in genres {
            let entry = stats.entry(genre.clone()).or_insert((0, *year, *year));
            entry.0 += count;
            entry.1 = entry.1.min(*year);
            entry.2 = entry.2.max(*year);
        }
    }

    stats
}


fn print_summary_statistics(stats: &HashMap<String, (usize, i32, i32)>) {
    println!("\nSummary Statistics:");
    println!("{:<20} {:<10} {:<10} {:<10}", "Genre", "Count", "Start Year", "End Year");
    println!("{:-<50}", "");
    for (genre, (count, start_year, end_year)) in stats {
        println!(
            "{:<20} {:<10} {:<10} {:<10}",
            genre, count, start_year, end_year
        );
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_release_year() {
        assert_eq!(parse_release_year("1995-07-14"), Some(1995));
        assert_eq!(parse_release_year(""), None);
        assert_eq!(parse_release_year("invalid-date"), None);
    }

    #[test]
    fn test_parse_genres() {
        let genres_json = r#"[{"id": 28, "name": "Action"}, {"id": 12, "name": "Adventure"}]"#;
        let parsed_genres = parse_genres(genres_json);
        assert_eq!(parsed_genres, vec!["Action", "Adventure"]);

        let invalid_json = r#"invalid-json"#;
        let parsed_genres = parse_genres(invalid_json);
        assert!(parsed_genres.is_empty());
    }
}