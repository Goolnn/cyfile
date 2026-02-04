use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::path::Path;
use zip::ZipArchive;

pub fn check_from_path<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();

    let file = match File::open(path) {
        Ok(val) => val,
        Err(_) => return false,
    };

    check_from_stream(file)
}

pub fn check_from_stream<R: Read + Seek>(reader: R) -> bool {
    let archive = match ZipArchive::new(reader) {
        Ok(val) => val,
        Err(_) => return false,
    };

    archive.comment() == b"Cangyan Project Package"
}
