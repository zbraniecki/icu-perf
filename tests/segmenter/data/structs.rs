use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Output {
    pub line: Box<[usize]>,
    pub word: Box<[usize]>,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct TestCase<'s> {
    pub langid: &'s str,
    pub input: &'s str,
    pub output: Output,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct TestData<'s>(
    #[serde(borrow)]
    pub Box<[TestCase<'s>]>
);
