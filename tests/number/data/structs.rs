use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LocaleOutput<'s> {
    pub langid: &'s str,
    pub value: &'s str,
}

#[derive(Serialize, Deserialize)]
pub struct TestCase<'s> {
    pub input: f64,
    #[serde(borrow)]
    pub output: Box<[(&'s str, &'s str)]>,
}

#[derive(Serialize, Deserialize)]
pub struct LocaleTestData<'s> {
    pub langid: Box<[&'s str]>,
    #[serde(borrow)]
    pub values: Vec<TestCase<'s>>,
}

#[derive(Serialize, Deserialize)]
pub struct TestData<'s>(
    #[serde(borrow)]
    pub Vec<LocaleTestData<'s>>
);
