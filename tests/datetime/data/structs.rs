use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct TestCase<'s> {
    pub input: i32,
    pub output: &'s str,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct LocaleTestData<'s> {
    pub langid: &'s str,
    #[serde(borrow)]
    pub values: Box<[TestCase<'s>]>,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct TestData<'s>(
    #[serde(borrow)]
    pub Box<[LocaleTestData<'s>]>
);
