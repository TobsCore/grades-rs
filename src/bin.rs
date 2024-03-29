use grades::Grade;
use std::env;
use std::io::{self, Read};

fn main() -> Result<(), std::io::Error> {
    let args = env::args().collect::<Vec<String>>();
    let input = if args.len() == 1 {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        parse_input(&buffer)
    } else {
        args.split_at(1).1.to_vec()
    };
    let mut grades = Vec::new();
    for arg in input {
        match Grade::from(&arg) {
            Ok(grade) => grades.push(grade),
            Err(_) => eprintln!("Ignoring {}, because it cannot be parsed", arg),
        }
    }

    if let Some(avg_grade) = grades::avg(&grades) {
        println!(
            "{1}, {0} <{2:.5}>",
            avg_grade,
            avg_grade.verbal(),
            grades::avg_prec(&grades).unwrap_or(0.0)
        );
    } else {
        eprintln!("Could not calculate average")
    }
    Ok(())
}

fn parse_input(buffer: &str) -> Vec<String> {
    buffer
        .split_ascii_whitespace()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
}
