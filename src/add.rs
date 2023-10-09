use csv::{Reader, StringRecord};
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};

fn add(filename: &str) {
    let data = fs::read_to_string(filename).expect("Unable to open csv file");
    let mut reader = Reader::from_reader(data.as_bytes());
}
