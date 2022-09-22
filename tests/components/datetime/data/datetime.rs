use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TestCase {
    pub input: i32,
    pub output: String,
}

#[derive(Serialize, Deserialize)]
pub struct LocaleTestData {
    pub langid: String,
    pub values: Vec<TestCase>,
}

#[derive(Serialize, Deserialize)]
pub struct TestData(pub Vec<LocaleTestData>);
