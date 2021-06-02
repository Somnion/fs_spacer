use std::fs;
use std::path::PathBuf;

extern crate libc;

use libc::*;
use std::ffi::CString;
use std::ffi::CStr;
use std::error::Error;

pub fn canonicalize_path(file_path: &String) -> std::io::Result<PathBuf>{
    return fs::canonicalize(file_path);
}

pub fn get_current_username() -> Result<String, String>{
    // should never fail
    let uid : libc::uid_t = unsafe { libc::getuid() };
    // Can fail
    let username = unsafe{ libc::getpwuid(uid) };
    if username.is_null(){
        return Err("Can't read current user name.".to_string());
    }

    let username =
        unsafe  { CStr::from_ptr((*username).pw_name).to_str().to_owned() };

    match username{
        Ok(uname ) => return Ok(uname.to_string()),
        Err(err) => return Err(err.to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::canonicalize_path;

    #[test]
    fn canonicalize() {
        let s = String::from("/Users/michaelkessler/Projects/streams");
        println!("{:?}", canonicalize_path(&s));
    }
}
