use fs_spacer::*;

use std::fs;

use fs_spacer::fs_zip;
use fs_spacer::fs_zip::{DailyLogFileEntry, zip_log_directory, extract_date_information};

extern crate libc;

fn main(){


    let stats = get_statvfs_for_path("/Users/michaelkessler");
    let stats = stats.unwrap();
    println!("{}", stats.f_bsize);

    let current_username= get_current_username();
    println!("{}", current_username.unwrap());

    /*let dir_struct = ConfigJSON{ dirs: vec![
        String::from("/Users/michaelkessler/Projects/streams/fs_spacer/test")]};

    let json = serde_json::to_string(&dir_struct);
    save_dirs("/Users/michaelkessler/Projects/streams/fs_spacer/test",
    "config.json", &json.unwrap());*/

    let content = read_dirs_from_file("./config.json");
    println!("{}", content.is_ok());


    zip_log_directory("/Users/michaelkessler/Projects/streams/fs_spacer/test/");
    // doit("/Users/michaelkessler/Projects/streams/fs_spacer/test/zippy.zip");
}
