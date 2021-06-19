use fs_spacer::{get_current_username, get_statvfs_for_path, SpaceUnit};
use std::os::raw::c_char;

extern crate libc;

use std::ffi::*;
use std::path::*;
use std::mem;
use std::os::unix::ffi::OsStrExt;

fn main(){
    /*let s = String::from("/Users/michaelkessler/Projects/streams");
    println!("{:?}", canonicalize_path(&s));*/

    /*let dir = Path::new("/Users/michaelkessler");
    let c_dir = CString::new(dir.as_os_str().as_bytes()).unwrap();
    unsafe{
        let mut stat: libc::statvfs = mem::zeroed();
        let ret: i32 = libc::statvfs(c_dir.as_ptr() as *const _, &mut stat);
        println!("{}", stat.f_bsize);
    }*/

    let stats = get_statvfs_for_path("/Users/michaelkessler");
    let stats = stats.unwrap();
    println!("{}", stats.f_bsize);


    let current_username= get_current_username();
    println!("{}", current_username.unwrap());


}
