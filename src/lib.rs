use std::fs;
use std::path::PathBuf;

pub fn canonicalize_path(file_path: &String) -> std::io::Result<PathBuf>{
    return fs::canonicalize(file_path);
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
