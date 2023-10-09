use csv::{Reader, StringRecord};
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};

macro_rules! log2 {
    ($val:expr) => {
        ($val as f32).log(2f32).ceil() as usize
    };
}

pub fn initialize(filename: &str) {
    let data = fs::read_to_string(filename).expect("Unable to open csv file");
    let mut reader = Reader::from_reader(data.as_bytes());
    let headers: StringRecord;
    {
        headers = reader.headers().expect("Unable to read headers").clone();
    }
    let mut data_for_enums: HashMap<&str, HashSet<String>> = HashMap::from_iter(
        headers
            .into_iter()
            .map(|header| (header, HashSet::new()))
            .collect::<Vec<_>>(),
    );
    for record in reader.records() {
        let record_length = record.as_ref().expect("Failed reading records").len();
        let rec = record.expect("Failed parsing record");
        for i in 0..record_length {
            match data_for_enums.get_mut(&headers[i]) {
                Some(values) => values.insert(rec[i].to_string()),
                None => panic!("Failed setting record"),
            };
        }
    }
    fs::create_dir(".temp");
    let column_sizes = headers
        .into_iter()
        .map(|header| {
            let log_val = log2!(data_for_enums
                .get(header)
                .expect("Couldn't find column")
                .len());
            if log_val == 0 {
                1
            } else {
                log_val
            }
        })
        .collect::<Vec<_>>();
    fs::write(
        ".temp/metadata",
        format!(
            "filename: {}\nbuffer size: {}\ncolumns indivial size: {:?}",
            filename,
            column_sizes.iter().sum::<usize>(),
            column_sizes
        ),
    )
    .expect("couldn't write temporary metadata");
    let mut arrs_file_contents: String;
    let mut variants: String = String::from("");
    let mut arrs: String = String::from("");
    let mut matches: String = String::from("");
    for (column_name, values) in data_for_enums {
        let uppercase_column_name = column_name
            .chars()
            .take(1)
            .flat_map(|c| c.to_uppercase())
            .chain(column_name.chars().skip(1))
            .collect::<String>();
        let values_as_vec = values
            .iter()
            .map(|val| {
                val.chars()
                    .take(1)
                    .flat_map(|c| c.to_uppercase())
                    .chain(val.chars().skip(1))
                    .collect::<String>()
            })
            .collect::<Vec<_>>();
        variants = format!("{}\n\t{},", variants, uppercase_column_name);
        arrs = format!("{}\n\tlet {} = {:?};", arrs, column_name, values_as_vec);
        matches = format!(
            "{}\n\t\tHeaders::{} => {},",
            matches, uppercase_column_name, column_name
        );
        arrs_file_contents = format!(
            r#"pub enum Headers {{
  {variants}
}}

pub fn get_arr<T>(header: Headers) -> [T] {{
  {arrs}

  match header{{
    {matches}
  }}
}}
"#
        );
        fs::write(".temp/arrs.rs", &arrs_file_contents.replace(" \n", ""));
    }
}
