use crate::{SORT_FIELD_ADDRESS, SORT_FIELD_NAME};
use serde::{Deserialize, Serialize};
use serde_json::{self};
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, Write};

pub struct TokenSorter {
    pub in_path: String,
    pub out_path: String,
    pub field: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct NameAndAddress {
    name: String,
    address: String,
}
impl TokenSorter {
    //
    pub fn is_valid_sort_field(field: &str) -> bool {
        field == SORT_FIELD_NAME || field == SORT_FIELD_ADDRESS
    }

    pub fn sort(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!(
            "Parsing file: {} using sort field: {} ",
            &self.in_path, &self.field
        );

        let file = File::open(&self.in_path)?;
        let reader = io::BufReader::new(file);

        let mut names_and_addresses: Vec<NameAndAddress> = Vec::new();
        for line in reader.lines() {
            let line = line.unwrap();
            let name_and_address: NameAndAddress = serde_json::from_str(&line)?;
            names_and_addresses.push(name_and_address);
        }

        if self.field == SORT_FIELD_NAME {
            names_and_addresses.sort_by_key(|s| s.name.clone());
        } else if self.field == SORT_FIELD_ADDRESS {
            names_and_addresses.sort_by_key(|s| s.address.clone());
        }

        let mut data: String = "".to_string();
        for record in names_and_addresses.into_iter() {
            let to_json = serde_json::to_string(&record).unwrap();
            data.push_str(to_json.as_str());
            data.push('\n');
        }
        data.pop();

        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .read(true)
            .write(true)
            .open(&self.out_path)?;
        file.write_all(data.as_ref())?;

        Ok(())
    }
}

#[allow(dead_code)]
const IN_PATH: &str = "./data/test_in";
#[allow(dead_code)]
const OUT_PATH: &str = "./data/test_out";

#[cfg(test)]
mod tests {
    use crate::token_sorter::{NameAndAddress, TokenSorter, IN_PATH, OUT_PATH};
    use crate::{SORT_FIELD_ADDRESS, SORT_FIELD_NAME};
    use std::fs::File;
    use std::io;
    use std::io::BufRead;

    #[test]
    fn test_is_valid_sort_field() {
        let mut field = "blah";
        assert!(!TokenSorter::is_valid_sort_field(field));

        field = "name";
        assert!(TokenSorter::is_valid_sort_field(field));

        field = "address";
        assert!(TokenSorter::is_valid_sort_field(field));
    }

    #[test]
    fn test_sort_by_name() {
        let mut ts = TokenSorter {
            in_path: IN_PATH.to_string(),
            out_path: OUT_PATH.to_string(),
            field: SORT_FIELD_NAME.to_string(),
        };
        ts.sort().unwrap();

        let count = get_lines_count();

        let file = File::open(OUT_PATH).unwrap();
        let lines = io::BufReader::new(&file).lines();

        let parse_line = |line: &str| {
            let name_and_address: NameAndAddress = serde_json::from_str(&line).unwrap();
            name_and_address
        };

        for (i, line) in lines.enumerate() {
            if i == 0 {
                let name_and_address: NameAndAddress = parse_line(&line.unwrap().as_str());
                assert_eq!(name_and_address.name, "Amp");
                assert_eq!(
                    name_and_address.address,
                    "0xfF20817765cB7f73d4bde2e66e067E58D11095C2"
                );
            } else if i == count - 1 {
                let name_and_address: NameAndAddress = parse_line(line.unwrap().as_str());
                assert_eq!(name_and_address.name, "hoge.finance");
                assert_eq!(
                    name_and_address.address,
                    "0xfAd45E47083e4607302aa43c65fB3106F1cd7607"
                );
            }
        }
    }

    #[test]
    fn test_sort_by_address() {
        let mut ts = TokenSorter {
            in_path: IN_PATH.to_string(),
            out_path: OUT_PATH.to_string(),
            field: SORT_FIELD_ADDRESS.to_string(),
        };
        ts.sort().unwrap();

        let count = get_lines_count();

        let file = File::open(OUT_PATH).unwrap();
        let lines = io::BufReader::new(&file).lines();

        let parse_line = |line: &str| {
            let name_and_address: NameAndAddress = serde_json::from_str(&line).unwrap();
            name_and_address
        };

        for (i, line) in lines.enumerate() {
            if i == 0 {
                let name_and_address: NameAndAddress = parse_line(line.unwrap().as_str());
                assert_eq!(name_and_address.name, "Reef.finance");
                assert_eq!(
                    name_and_address.address,
                    "0xFE3E6a25e6b192A42a44ecDDCd13796471735ACf"
                );
            } else if i == count - 1 {
                let name_and_address: NameAndAddress = parse_line(line.unwrap().as_str());
                assert_eq!(name_and_address.name, "FalconSwap Token");
                assert_eq!(
                    name_and_address.address,
                    "0xfffffffFf15AbF397dA76f1dcc1A1604F45126DB"
                );
            }
        }
    }

    fn get_lines_count() -> usize {
        let file = File::open(OUT_PATH).unwrap();
        io::BufReader::new(&file).lines().count()
    }
}
