use fs_spacer::*;

use std::fs;

use fs_spacer::fs_zip;
use fs_spacer::fs_zip::DailyLogFileEntry;

extern crate libc;

fn main(){


    let stats = get_statvfs_for_path("/Users/michaelkessler");
    let stats = stats.unwrap();
    println!("{}", stats.f_bsize);

    let current_username= get_current_username();
    println!("{}", current_username.unwrap());

    println!("Is match: {}", yyyy_mm_dd_match("hallo-2021-06-22.log"));

    /*let dir_struct = ConfigJSON{ dirs: vec![
        String::from("/Users/michaelkessler/Projects/streams/fs_spacer/test")]};

    let json = serde_json::to_string(&dir_struct);
    save_dirs("/Users/michaelkessler/Projects/streams/fs_spacer/test",
    "config.json", &json.unwrap());*/

    let content = read_dirs_from_file("./config.json");
    println!("{}", content.is_ok());

    fs_zip::hello();

    let mut v = vec![DailyLogFileEntry::new(2021, 06, 03, "B"),
                 DailyLogFileEntry::new(2021, 06, 01, "D"),
                 DailyLogFileEntry::new(2021, 06, 01, "A"),
                 DailyLogFileEntry::new(2021, 04, 02, "E")];

    v.sort();
    for item in v {
        println!("{:?}", item);
    }

    /*let path = fs::canonicalize("./config.json");
    println!("{}", path.unwrap().display());*/
}
