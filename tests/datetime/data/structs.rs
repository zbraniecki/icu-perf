use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Value<'s> {
    pub epoch: i32,
    #[serde(borrow)]
    pub output: Box<[(&'s str, &'s str)]>,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct TestCase<'s> {
    pub style: (&'s str, &'s str),
    #[serde(borrow)]
    pub value: Box<[Value<'s>]>,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct LocaleTestData<'s> {
    #[serde(borrow)]
    pub langid: Box<[&'s str]>,
    #[serde(borrow)]
    pub cases: Box<[TestCase<'s>]>,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct TestData<'s>(
    #[serde(borrow)]
    pub Box<[LocaleTestData<'s>]>
);
