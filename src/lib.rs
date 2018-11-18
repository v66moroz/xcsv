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

type CSVIterType = Iterator<Item=Result<csv::StringRecord, csv::Error>>;

struct CSVIter {
    iter: Box<CSVIterType>,
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
        write!(f, "CSVIter")
    }
}

impl Clone for CSVIter {
    fn clone(&self) -> CSVIter { 
        panic!("Not cloneable!") 
    }
}

ruby! {
    class XCSV {
        struct {
            path: String,
            iter: Option<CSVIter>,
        }

        def initialize(helix, path: String) {
            XCSV { helix, path, iter: None }
        }

        def open(&mut self) -> Result<(), helix::Error> {
            self.iter = None;

            let gz_regex = Regex::new("\\.gz\\z").unwrap();

            let buf_reader = 
                match File::open(&self.path) {
                    Ok(f)   => BufReader::new(f),
                    Err(e)  => raise!(e.to_string()),
                };

            let gz_reader: Box<Read> =
                if gz_regex.is_match(&self.path) {
                    Box::new(GzDecoder::new(buf_reader)) 
                } else {
                    Box::new(buf_reader)  
                };

            let csv_reader =
                csv::ReaderBuilder::new()
                    .has_headers(false)
                    .from_reader(gz_reader);

            self.iter = Some(CSVIter{iter: Box::new(csv_reader.into_records())});
            Ok(())
        }

        def next(&mut self) -> Result<Option<Vec<String>>, helix::Error> {
            match self.iter {
                Some(ref mut iter) =>
                    match iter.next() {
                        Some(Ok(record)) =>
                            Ok(Some(record.iter().map(|s| s.to_string()).collect())), 
                        Some(Err(e)) =>
                            raise!(e.to_string()),
                        None =>
                            Ok(None)
                    }
                None =>
                    raise!("closed file")
            }
        }

        def close(&mut self) -> () {
            self.iter = None
        }
    }
}
