use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

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
    let mut file = File::open(path)?;
    let mut cities: HashMap<String, Stats> = HashMap::new();

    const BUFFER_SIZE: usize = 2 * 1024 * 1024;
    let mut buffer = vec![0u8; BUFFER_SIZE];
    let mut end: usize = 0;

    loop {
        // Read into the unused portion of the buffer.
        let bytes_read = file.read(&mut buffer[end..])?;

        if bytes_read == 0 {
            // EOF: process any remaining unterminated line.
            if end > 0 {
                let line_end = end;
                process_line(&buffer[..line_end], &mut cities);
            }

            break;
        }

        end += bytes_read;

        let mut start: usize = 0;

        // Process every complete line.
        while let Some(delim_idx) = buffer[start..end].iter().position(|&b| b == b'\n') {
            let line_end = start + delim_idx;

            process_line(&buffer[start..line_end], &mut cities);

            // Move to the next line.
            start = line_end + 1;
        }

        // Shift incomplete line bytes to the beginning.
        if start > 0 {
            buffer.copy_within(start..end, 0);
            end -= start;
        }
    }

    for (city, stats) in cities {
        println!("{} average temp {}", city, stats.total / stats.count as f64);
    }

    Ok(())
}

fn process_line(line: &[u8], cities: &mut HashMap<String, Stats>) {
    let semi = line.iter().position(|&b| b == b';').unwrap();

    let city = std::str::from_utf8(&line[..semi]).unwrap();
    let temp: f64 = std::str::from_utf8(&line[semi + 1..])
        .unwrap()
        .parse()
        .unwrap();

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
