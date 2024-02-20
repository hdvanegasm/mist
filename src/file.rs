use std::fs;
use std::io::Result;

struct File {
    bytes: Vec<u8>,
    path: String,
}

impl File {
    fn read_from_path(path: String) -> Result<Self> {
        let file_bytes = fs::read(path)?;
        File {
            bytes: file_bytes,
            path: path,
        }
    }
}