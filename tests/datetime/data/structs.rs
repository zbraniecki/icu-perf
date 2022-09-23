use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct TestCase<'s> {
    pub input: i32,
    pub style: (&'s str, &'s str),
    pub output: &'s str,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct LocaleTestData<'s> {
    #[serde(borrow)]
    pub langid: Box<[&'s str]>,
    #[serde(borrow)]
    pub values: Box<[TestCase<'s>]>,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct TestData<'s>(
    #[serde(borrow)]
    pub Box<[LocaleTestData<'s>]>
);
