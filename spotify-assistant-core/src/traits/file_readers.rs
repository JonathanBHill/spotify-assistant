use std::path::PathBuf;

pub trait ConfigReader {
    fn file_path() -> PathBuf;
    fn new() -> Self;
}
