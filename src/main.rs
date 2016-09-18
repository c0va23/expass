use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::{
    BTreeMap,
    BTreeSet,
};

fn main() {
    let file = File::open("expass.csv").unwrap();
    let reader = BufReader::new(file);
    let mut passports = BTreeMap::new();
    for buffer in reader.lines() {
        let line = buffer.unwrap();
        let series: u16 = line[0..3].parse().unwrap();
        let number: u32 = line[5..11].parse().unwrap();
        passports.entry(series)
                .or_insert(BTreeSet::new())
                .insert(number);
    }
    for (series, numbers) in passports {
        println!("Series {:04} have {} numbers", series, numbers.len());
    }
}
