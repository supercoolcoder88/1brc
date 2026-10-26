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
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        panic!("missing args");
    }

    if args[1] == "1b" {
        get_stats("source-1brc/measurements.txt".to_string())?;
        return Ok(());
    } else if args[1] == "1m" {
        get_stats("source-1brc/1m-measurements.txt".to_string())?;
        return Ok(());
    }

    panic!("Bad args");
}

fn get_stats(path: String) -> std::io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut cities: HashMap<String, Stats> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split(';');
        let city = parts.next().unwrap_or("");
        let temp: f64 = parts.next().unwrap_or("").parse().unwrap();

        if let Some(e) = cities.get_mut(city) {
            e.count += 1;
            e.total += temp;
            e.max = e.max.max(temp);
            e.min = e.min.min(temp);
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

    for k in cities {
        print!("{} average temp {}", k.0, k.1.total / k.1.count as f64);
    }

    Ok(())
}
