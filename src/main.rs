use fs_spacer::*;

extern crate libc;

fn main(){


    let stats = get_statvfs_for_path("/Users/michaelkessler");
    let stats = stats.unwrap();
    println!("{}", stats.f_bsize);

    let current_username= get_current_username();
    println!("{}", current_username.unwrap());

    println!("Is match: {}", yyyy_mm_dd_match("hallo-2021-06-22.log"));

    let dir_struct = ConfigJSON{ dirs: vec![
        String::from("/Users/michaelkessler/Projects/streams/fs_spacer/test")]};

    let json = serde_json::to_string(&dir_struct);
    save_dirs("/Users/michaelkessler/Projects/streams/fs_spacer/test",
    "config.json", &json.unwrap());
}
