use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn group_sums(lines: impl Iterator<Item = io::Result<String>>) -> io::Result<Vec<i32>> {
    let mut v = Vec::new();

    let mut current_sum: i32 = 0;

    for (line_number, line_or_error) in lines.enumerate() {
        let line = line_or_error?;

        if line == "" {
            v.push(current_sum);
            current_sum = 0;
            continue;
        }

        let n = line.parse::<i32>()
            .expect(&format!("Parse error on line {}: {}", line_number + 1, line));
        current_sum += n;
    }

    v.push(current_sum);

    Ok(v)
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let mut v: Vec<i32> = group_sums(reader.lines())?;

    v.select_nth_unstable_by_key(2, |a| -a);
    println!("{}", v[0..=2].iter().sum::<i32>());

    Ok(())
}
