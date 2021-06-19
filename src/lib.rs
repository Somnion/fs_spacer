use std::fs;
use std::path::PathBuf;

extern crate libc;

use libc::*;
use std::ffi::CString;
use std::ffi::CStr;
use std::error::Error;

use std::path::*;
use std::mem;
use std::os::unix::ffi::OsStrExt;


#[non_exhaustive]
pub struct SpaceUnit;

impl SpaceUnit {
    pub const KB: u64 = 1024;
    pub const MB: u64 = 1024 * 1024;
    pub const GB: u64 = 1024 * 1024 * 1024;
    pub const TB: u64 = 1024 * 1024 * 1024 * 1024;
}

pub fn get_current_username() -> Result<String, String>{
    // should never fail
    let uid : libc::uid_t = unsafe { libc::getuid() };
    // Can fail
    let username = unsafe{ libc::getpwuid(uid) };
    // C-pointer, can therefor be null
    if username.is_null(){
        return Err("Can't read current user name.".to_string());
    }
    // to_owned() creates a copy of the username, separating it from the C-pointer
    let rustified_username =
        unsafe  { CStr::from_ptr((*username).pw_name).to_str().to_owned() };

    return match rustified_username {
        Ok(uname) => Ok(uname.to_string()),
        Err(err) => Err(err.to_string()),
    }
}

pub fn get_statvfs_for_path(dir_path: &str) -> Result<libc::statvfs, String> {
    let dir = Path::new(dir_path);
    if !dir.exists() {
        return Err(format!("{} doesn't exist as a directory.", dir_path));
    }

    let dir_as_c_representation = CString::new(dir.as_os_str().as_bytes()).unwrap();
    unsafe {
        let mut stat: libc::statvfs = mem::zeroed();
        return match libc::statvfs(dir_as_c_representation.as_ptr() as *const _,
                                     &mut stat){
            0 => Ok(stat),
            _ => Err(format!("Could get stats for dir {}", dir_path)),
        }
    }
}

pub fn get_formatted_space(space: u64) -> Result<String, String> {
    match space {
        size if size < SpaceUnit::KB => Ok(format!("{} B", size)),
        size if size < SpaceUnit::MB => Ok(format!("{} KB", size as u64 / SpaceUnit::KB as u64)),
        size if size < SpaceUnit::GB => Ok(format!("{} MB", size as u64 / SpaceUnit::MB as u64)),
        size if size < SpaceUnit::TB => Ok(format!("{} GB", size as u64 / SpaceUnit::GB as u64)),
        size if size >= SpaceUnit::TB => Ok(format!("{} TB", size as u64 / SpaceUnit::TB as u64)),
        _ => Err(format!("Can't process size {} for formatted output", space)),
    }
}

/*#[cfg(test)]
mod test {
    use crate::canonicalize_path;

    #[test]
    fn canonicalize() {
        let s = String::from("/Users/michaelkessler/Projects/streams");
        println!("{:?}", canonicalize_path(&s));
    }
}*/
