use std::fs::File;
use std::io::Write;

pub fn write_file(filename: &String, content: &[u8]) {
    let mut file = File::create_new(filename);
    file.as_mut().expect("").write_all(content).unwrap();
}
