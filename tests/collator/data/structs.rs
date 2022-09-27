use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TestCase<'s> {
    pub left: &'s str,
    pub right: &'s str,
    pub output: &'s str,
}

#[derive(Serialize, Deserialize)]
pub struct LocaleTestData<'s> {
    pub langid: &'s str,
    #[serde(borrow)]
    pub values: Vec<TestCase<'s>>,
}

#[derive(Serialize, Deserialize)]
pub struct TestData<'s>(
    #[serde(borrow)]
    pub Vec<LocaleTestData<'s>>
);
