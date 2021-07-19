use std::io::prelude::*;
use std::io::{Seek, Write};
use std::iter::Iterator;
use zip::result::ZipError;
use zip::write::FileOptions;

use std::fs::File;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

use std::cmp::Ordering;

pub fn hello(){
    println!("Hello .rs")
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DailyLogFileEntry{
    // Encapsulates the log file for a given day
    year: i32,
    month: i8,
    day: i8,
    logfile_name: String,
}

impl DailyLogFileEntry{
    pub fn new(year: i32, month: i8, day: i8, logfile_name: &str) -> Self {
        DailyLogFileEntry{
            year: year,
            month: month,
            day: day,
            logfile_name: String::from(logfile_name),
        }
    }
}

/*impl PartialEq for DailyLogFileEntry {
    fn eq(&self, other: &Self) -> bool {
        match self.year == other.year{
            true => {
                match self.month == other.month {
                    true => self.day == other.day,
                    false => false,
                }
            },
            false => false,
        }
    }
}*/

const METHOD_STORED: Option<zip::CompressionMethod> = Some(zip::CompressionMethod::Stored);

#[cfg(any(
feature = "deflate",
feature = "deflate-miniz",
feature = "deflate-zlib"
))]
const METHOD_DEFLATED: Option<zip::CompressionMethod> = Some(zip::CompressionMethod::Deflated);
#[cfg(not(any(
feature = "deflate",
feature = "deflate-miniz",
feature = "deflate-zlib"
)))]
const METHOD_DEFLATED: Option<zip::CompressionMethod> = None;

#[cfg(feature = "bzip2")]
const METHOD_BZIP2: Option<zip::CompressionMethod> = Some(zip::CompressionMethod::Bzip2);
#[cfg(not(feature = "bzip2"))]
const METHOD_BZIP2: Option<zip::CompressionMethod> = None;