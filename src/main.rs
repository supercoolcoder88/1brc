use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Stats {
    min: f64,
    max: f64,
    total: f64,
    count: i32,
}

fn main() -> std::io::Result<()> {
    let cities = get_stats("source-1brc/measurements.txt".to_string())?;
    for (city, stats) in &cities {
        let avg = stats.total / stats.count as f64;
        println!("{}={:.1}/{:.1}/{:.1}", city, stats.min, avg, stats.max);
    }
    println!("number of cities={}", cities.len());
    Ok(())
}

fn get_stats(path: String) -> std::io::Result<HashMap<String, Stats>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut cities: HashMap<String, Stats> = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split(';');
        let city = parts.next().unwrap_or("");
        let temp: f64 = parts.next().unwrap_or("").parse().unwrap();

        if let Some(stats) = cities.get_mut(city) {
            stats.count += 1;
            stats.total += temp;
            stats.max = stats.max.max(temp);
            stats.min = stats.min.min(temp);
        } else {
            cities.insert(
                city.to_string(),
                Stats {
                    min: temp,
                    max: temp,
                    count: 1,
                    total: temp,
                },
            );
        }
    }

    Ok(cities)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let res = get_stats("measurements.txt".to_string()).unwrap();

        let sydney = res.get("Sydney").unwrap();
        assert_eq!(sydney.min, 18.0);
        assert_eq!(sydney.max, 25.0);
        assert_eq!(sydney.total, 64.0);
        assert_eq!(sydney.count, 3);
    }
}
