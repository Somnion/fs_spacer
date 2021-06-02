use fs_spacer::{canonicalize_path, get_current_username};
use std::os::raw::c_char;

extern crate libc;

use std::ffi::*;
use std::path::*;
use std::mem;
use std::os::unix::ffi::OsStrExt;

fn main(){
    let s = String::from("/Users/michaelkessler/Projects/streams");
    println!("{:?}", canonicalize_path(&s));

    let dir = Path::new("/Users/michaelkessler");
    let c_dir = CString::new(dir.as_os_str().as_bytes()).unwrap();
    unsafe{
        let mut stat: libc::statvfs = mem::zeroed();
        let ret: i32 = libc::statvfs(c_dir.as_ptr() as *const _, &mut stat);
        println!("{}", stat.f_bsize);
    }

    let x= get_current_username();
    println!("{}", x.unwrap());
}
