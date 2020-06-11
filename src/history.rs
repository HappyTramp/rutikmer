use std::time::Duration;
use std::str;
use std::num;
use std::fmt;

use csv;
use chrono;
use chrono::prelude::*;

use super::scramble::Scramble;


pub struct SolveTime(Duration);

impl str::FromStr for SolveTime {
    type Err = num::ParseIntError;
    // fmt: mm:ss.lll  (l = millisecond)
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let minute: u64 = s[..2].parse()?;
        let second: u64 = s[3..5].parse()?;
        let millis: u64 = s[6..].parse()?;
        Ok(SolveTime(Duration::from_secs(minute * 60_u64 + second) + Duration::from_millis(millis)))
    }
}

impl fmt::Display for SolveTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}.{:0>3}",
               self.0.as_secs() / 60,
               self.0.as_secs() % 60,
               self.0.as_millis() % 1000)
    }
}

struct Entry {
    time: SolveTime,
    scramble: Scramble,
    date: chrono::DateTime<Utc>,
}

pub struct History(Vec<Entry>);

const VEC_START_SIZE: usize = 200;

impl History {
    pub fn from_csv(file_path: &str) -> History {
        let mut history = History(Vec::with_capacity(VEC_START_SIZE));
        let mut reader = csv::Reader::from_path(file_path).unwrap();
        for result in reader.records() {
            if let Ok(record) = result {
                history.0.push(Entry{
                    time:     record[0].parse::<SolveTime>().unwrap(),
                    scramble: record[1].parse::<Scramble>().unwrap(),
                    date:     record[2].parse::<chrono::DateTime<Utc>>().unwrap()
                })
            }
        }
        history
    }

    pub fn save_csv(&self, file_path: &str) {
        let mut writter = csv::Writer::from_path(file_path).unwrap();
        writter.write_record(&["time", "scramble", "date"]).unwrap();
        for entry in &self.0 {
            writter.write_record(&[
                entry.time.to_string(),
                entry.scramble.to_string(),
                entry.date.to_string()
            ]).unwrap();
        }
        writter.flush().unwrap();
    }

    pub fn summary(&self, n: usize) -> Vec<String> {
        self.0.iter().skip(self.0.len() - n).map(|Entry{time, ..}| time.to_string()).collect()
    }
}
