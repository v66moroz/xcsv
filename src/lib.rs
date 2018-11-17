extern crate csv;
extern crate flate2;
extern crate regex;

#[macro_use]
extern crate helix;

use std::fs::File;
use std::io::{BufReader, Read};
use std::ops::{Deref, DerefMut};

use flate2::read::GzDecoder;
use regex::Regex;

use helix::{FromRuby, CheckResult};
use helix::sys::{VALUE};

type CSVIterType = Iterator<Item=Result<csv::StringRecord, csv::Error>>;

struct CSVIter {
    iter: Box<CSVIterType>,
    path: String,
}

impl Deref for CSVIter {
    type Target = CSVIterType;

    fn deref(&self) -> &CSVIterType {
        &self.iter
    }
}

impl DerefMut for CSVIter {
    fn deref_mut(&mut self) -> &mut CSVIterType {
        &mut self.iter
    }
}

impl std::fmt::Debug for CSVIter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "CSVIter: {}", self.path)
    }
}

impl Clone for CSVIter {
    fn clone(&self) -> CSVIter { 
        panic!("Not cloneable!") 
    }
}

impl FromRuby for CSVIter {
    type Checked = CSVIter;

    fn from_ruby(value: VALUE) -> CheckResult<CSVIter> {
        let checked_path = String::from_ruby(value)?;

        let path = String::from_checked(checked_path);

        let gz_regex = Regex::new("\\.gz\\z").unwrap();

        let buf_reader = 
            match File::open(path.clone()) {
                Ok(f)   => BufReader::new(f),
                Err(e)  => raise!(format!("Error while opening file: {}", e)),
            };

        let gz_reader: Box<Read> =
            if gz_regex.is_match(&path) {
                Box::new(GzDecoder::new(buf_reader)) 
            } else {
                Box::new(buf_reader)  
            };

        let csv_reader =
            csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(gz_reader);

        Ok(CSVIter{iter: Box::new(csv_reader.into_records()), path: path})
    }

    fn from_checked(checked: CSVIter) -> CSVIter {
        checked
    }
}

ruby! {
    class XCSV {
        struct {
            iter: CSVIter,
        }

        def initialize(helix, iter: CSVIter) {
            XCSV { helix, iter }
        }

        def next(&mut self) -> Result<Option<Vec<String>>, helix::Error> {
            match self.iter.next() {
                Some(Ok(record)) =>
                    Ok(Some(record.iter().map(|s| s.to_string()).collect())), 
                Some(Err(e)) =>
                    raise!(e.to_string()),
                None =>
                    Ok(None)
            }
        }
    }
}
