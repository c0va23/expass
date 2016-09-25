use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::{
    BTreeMap,
    BTreeSet,
};
use std::time::{
    Instant,
};

pub struct Database {
    passports: BTreeMap<u16, BTreeSet<u32>>,
}

fn parse_line(line: String) -> Result<(u16, u32), String> {
    let mut line_parts = line.splitn(2, ",");
    match line_parts.next().unwrap().parse() {
        Ok(series) => {
            match line_parts.next().unwrap().parse() {
                Ok(number) => Ok((series, number)),
                Err(err) => Err(format!("Invalid number: {}", err)),
            }
        },
        Err(err) => Err(format!("Invalid series: {}", err)),
    }
}

fn parse_passports(file_path: &str) -> BTreeMap<u16, BTreeSet<u32>> {
    println!("Start parse file {}", file_path);
    let instant = Instant::now();
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut passports = BTreeMap::new();
    for buffer in reader.lines().skip(1) {
        let line = buffer.unwrap();
        match parse_line(line.to_string()) {
            Ok((series, number)) => {
                passports.entry(series)
                    .or_insert(BTreeSet::new())
                    .insert(number);
            },
            Err(err) =>
                println!("Error parse line {} with error: {}", line, err),
        }
    }
    println!("Number of series {}", passports.len());
    let parse_duration = instant.elapsed();
    println!("Parse duration {:?}", parse_duration.as_secs());
    passports
}

impl Database {
    pub fn new(file_path: &str) -> Database {
        Database {
            passports: parse_passports(file_path),
        }
    }

    pub fn is_exist(&self, series: u16, number: u32) -> bool {
        self.passports.contains_key(&series) &&
            self.passports.get(&series).unwrap().contains(&number)
    }
}
