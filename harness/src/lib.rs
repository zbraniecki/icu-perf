pub mod data;
pub mod structs;

pub mod datetime;
pub mod locale;
pub mod plurals;

use serde::de;
use std::fs::File;
use std::io::BufReader;

pub fn get_fixture<T>(path: &str) -> std::io::Result<T>
where
    T: de::DeserializeOwned,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(serde_json::from_reader(reader)?)
}

pub trait HarnessImplementation {
    fn run_app();
}
