extern crate csv;

#[macro_use]
extern crate helix;

use std::fs::File;
use std::io::BufReader;
use std::ops::{Deref, DerefMut};

use helix::{FromRuby, CheckResult};
use helix::sys::{VALUE};

type CSVIterType = Iterator<Item=Result<csv::StringRecord, csv::Error>>;

struct CSVIter {
    iter: Box<CSVIterType>
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

impl FromRuby for CSVIter {
    type Checked = CSVIter;

    fn from_ruby(value: VALUE) -> CheckResult<CSVIter> {
        let checked_path = String::from_ruby(value)?;

        let csv_reader = 
            match File::open(String::from_checked(checked_path)) {
                Ok(f) => 
                    BufReader::new(f),
                Err(e) =>
                    raise!(format!("Error while opening file: {}", e)),
            };

        let csv_reader =
            csv::ReaderBuilder::new()
                .has_headers(false)
                .from_reader(csv_reader);

        let records = csv_reader.into_records();

        Ok(CSVIter{iter: Box::new(records)})
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

        def next_line(&mut self) -> Result<Option<Vec<String>>, helix::Error> {
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
