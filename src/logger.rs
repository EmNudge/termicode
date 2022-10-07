use std::{env, fs, io::prelude::*};

pub fn log<S: AsRef<str>>(text: S) {
    let mut temp_dir = env::temp_dir();
    temp_dir.push("termicode");
    if !temp_dir.exists() {
        fs::create_dir(&temp_dir).unwrap();
    }
    temp_dir.push("logs.txt");

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(temp_dir)
        .unwrap();

    file.write_all(text.as_ref().as_bytes()).unwrap();
    file.write_all(b"\n").unwrap();
}
