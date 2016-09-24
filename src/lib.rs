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
    passports: BTreeMap<String, BTreeSet<u32>>,
}

fn parse_line(line: String) -> Result<(String, u32), String> {
    let mut line_parts = line.splitn(2, ",");
    let series = line_parts.next().unwrap().to_string();
    let number_str = line_parts.next().unwrap();
    match number_str.parse() {
        Ok(number) => Ok((series, number)),
        Err(err) => Err(format!("{}", err)),
    }
}

fn parse_passports(file_path: &str) -> BTreeMap<String, BTreeSet<u32>> {
    println!("Start parse {}", file_path);
    let instant = Instant::now();
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut passports = BTreeMap::new();
    for buffer in reader.lines() {
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

    pub fn is_exist(&self, series: String, number: u32) -> bool {
        self.passports.contains_key(&series) &&
            self.passports.get(&series).unwrap().contains(&number)
    }
}
