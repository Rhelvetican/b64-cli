use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub fn read_file<T: Into<PathBuf>>(path: T) -> Result<Vec<String>> {
    let path = path.into();
    let reader = BufReader::new(File::open(path)?);
    Ok(reader.lines().collect::<Result<Vec<String>, _>>()?)
}
