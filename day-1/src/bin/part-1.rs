use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let mut max_sum: i32 = 0;
    let mut current_sum: i32 = 0;

    for (line_number, line_or_error) in reader.lines().enumerate() {
        let line = line_or_error?;

        if line == "" {
            if current_sum > max_sum {
                max_sum = current_sum;
            }
            current_sum = 0;
            continue;
        }

        let n = line.parse::<i32>()
            .expect(&format!("Parse error on line {}: {}", line_number + 1, line));
        current_sum += n;
    }

    if current_sum > max_sum {
        max_sum = current_sum;
    }

    println!("{}", max_sum);
    Ok(())
}
