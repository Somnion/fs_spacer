use std::io::prelude::*;
use std::io::{Seek, Write};
use std::iter::Iterator;
use zip::result::{ZipError, ZipResult};
use zip::write::FileOptions;

use std::fs::File;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};
use std::time::{Duration, SystemTime};

use std::cmp::Ordering;

use lazy_static::lazy_static;

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use zip::{CompressionMethod, ZipWriter};

const CHUNKSIZE : usize = 0x4000;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Hash)]
pub struct DailyLogFileEntry{
    // Encapsulates the log file for a given day
    year: String,
    month: String,
    day: String,
    logfile_name: String,
}

impl DailyLogFileEntry{
    pub fn new(year: &str, month: &str, day: &str, logfile_name: &str) -> Self {
        DailyLogFileEntry{
            year: year.parse().unwrap(),
            month: month.parse().unwrap(),
            day: day.parse().unwrap(),
            logfile_name: String::from(logfile_name),
        }
    }
}

pub fn yyyy_mm_dd_match(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r".*(\d{4})\-(\d{2})\-(\d{2})\.log").unwrap();
    }
    RE.is_match(text)
}

pub fn extract_date_information(input: &str) -> Option<DailyLogFileEntry> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r".*(\d{4})\-(\d{2})\-(\d{2})\.log").unwrap();
    }
    // Sanity: Filename must include specific date format
    if yyyy_mm_dd_match(input) == false {
        return None;
    }

    if let Some(caps) = RE.captures(input) {
        return Some(DailyLogFileEntry::new(caps.get(1).unwrap().as_str(),
                                             caps.get(2).unwrap().as_str(),
                                               caps.get(3).unwrap().as_str(),
                                           caps.get(0).unwrap().as_str()));
    }

    None
}

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

pub fn zip_log_directory(src_dir: &str) -> zip::result::ZipResult<()> {

    // SANITY: Source directory must exist otherwise fail fast
    if !Path::new(src_dir).is_dir() {
        return Err(ZipError::FileNotFound);
    }

    let method = Some(zip::CompressionMethod::Deflated);
    let walkdir = WalkDir::new(src_dir.to_string());
    let it = walkdir.into_iter();

    let mut file_list_vec: Vec<DailyLogFileEntry> = it
        .filter_map(|e| is_matching_file(&e.unwrap()))
        .collect();

    // Nothing to do if not files to zip
    if file_list_vec.len() == 0 {
        return Ok(());
    }

    // Sort by filename
    file_list_vec.sort();

    let mut groupedEntries =
        group_files_by_date_interval(&file_list_vec);


    zip_files(&groupedEntries, src_dir, &method);


    return Ok(())
}

fn zip_files(grouped_files: &HashMap<std::string::String, Vec<DailyLogFileEntry>>,
            src_dir: &str, method: &Option<CompressionMethod>) -> zip::result::ZipResult<()>{

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let src_path = Path::new(src_dir);

    for group in grouped_files.iter(){

        let pathname = format!("{}{}.zip", src_dir, group.0);
        println!("Starting group: {}", pathname);
        let path = Path::new(&pathname);
        let dst_file = File::create(&path).unwrap();

        let mut zip = zip::ZipWriter::new(dst_file);
        //zip.add_directory("/", Default::default());


        for entry in group.1{
            // let mut buffer = Vec::new();
            let entry_logfile_path = format!("{}{}", src_dir, entry.logfile_name).to_string();
            let entry_logfile_path = Path::new(&entry_logfile_path);
            let name = entry_logfile_path.strip_prefix(src_path).unwrap();

            let mut f = File::open(entry_logfile_path)?;

            let target = format!("{}{}", "", name.to_str().unwrap());
            zip.start_file(target, options)?;
            write_file_content_to_zip(&mut zip, &f)?;
        }
        zip.finish()?;
    }
    Result::Ok(())
}

fn write_file_content_to_zip(zip: &mut ZipWriter<File>, source: &File) -> zip::result::ZipResult<()> {

    let br = std::io::BufReader::new(source);

    loop
    {
        let mut buffer: Vec<u8> = Vec::with_capacity(CHUNKSIZE);

        let bytes_read = br.get_ref().take(CHUNKSIZE as u64).read_to_end(&mut buffer)?;

        if bytes_read == 0 {
            break;
        }

        zip.write_all(&buffer[..bytes_read])?;

        let s = std::str::from_utf8(&buffer[..bytes_read]);

        if bytes_read < CHUNKSIZE {
            break;
        }
        // drop(reference);
    }
    Result::Ok(())
}

fn group_files_by_date_interval(filelist: &Vec<DailyLogFileEntry>) -> HashMap<String, Vec<DailyLogFileEntry>>{
    // assert_ne!(filelist.len() == 0);
    let mut latest = &filelist[0];
    let mut groupedEntries: HashMap<String, Vec<DailyLogFileEntry>> = HashMap::new();
    let mut vecGroupedEntries:Vec<DailyLogFileEntry> = Vec::new();

    let mut groupId = format!("{}-{}-{}", latest.year, latest.month, latest.day.split_at(1).0);

    for i in 0..filelist.len() {
        let current = &filelist[i];
        match current{
            _ if latest.year != current.year ||
                latest.month != current.month ||
                latest.day.split_at(1).0 != current.day.split_at(1).0 => {
                groupedEntries.insert(groupId.clone(), vecGroupedEntries.clone());
                // create new group
                latest = current;
                groupId = format!("{}-{}-{}", latest.year, latest.month, latest.day.split_at(1).0);
                vecGroupedEntries = Vec::new();
                vecGroupedEntries.push(current.clone());
            },
            _ => {
                vecGroupedEntries.push(current.clone());
            }
        }
    }

    if vecGroupedEntries.len() != 0{
        groupedEntries.insert(groupId.clone(), vecGroupedEntries.to_vec());
    }

    groupedEntries
}

fn is_matching_file(entry: &DirEntry) -> Option<DailyLogFileEntry> {
    match is_last_modified_10_days_or_more(entry) {
        Ok(true) => (),
        Ok(false) => return None,
        Err(msg) => {
            eprintln!("{}", msg);
            return None;
        },
    }
    let entry = entry.file_name().to_str().unwrap();
    extract_date_information(entry)
}

fn is_last_modified_10_days_or_more(entry: &DirEntry) -> Result<bool, String>{
    let time_diff = Duration::new(10 * 60 * 60 * 24, 0);
    let now = SystemTime::now();

    match entry.metadata() {
        Ok(metadata) => {
            match metadata.modified() {
                Ok(modified) => {
                    return if (modified + time_diff) < now {
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                },
                _ => {
                    return Err(format!("Couldn't read modified property for {:?}",
                                       entry.file_name().to_str().unwrap()));
                },
            }
        }
        _ => return Err(format!("Couldn't read metadata for {:?}",
                                entry.file_name().to_str().unwrap())),
    }

}