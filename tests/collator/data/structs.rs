use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TestValue<'s> {
    pub left: &'s str,
    pub right: &'s str,
    #[serde(borrow)]
    pub output: Box<[(&'s str, &'s str)]>,
}

#[derive(Serialize, Deserialize)]
pub struct TestCase<'s> {
    #[serde(borrow)]
    pub values: Box<[TestValue<'s>]>
}

#[derive(Serialize, Deserialize)]
pub struct LocaleTestData<'s> {
    #[serde(borrow)]
    pub langids: Box<[&'s str]>,
    #[serde(borrow)]
    pub cases: Box<[TestCase<'s>]>,
}

#[derive(Serialize, Deserialize)]
pub struct TestData<'s>(
    #[serde(borrow)]
    pub Vec<LocaleTestData<'s>>
);
