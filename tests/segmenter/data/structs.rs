use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct DivergentOutput {
    pub line: Box<[usize]>,
    pub word: Box<[usize]>,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Output {
    pub line: Box<[usize]>,
    pub word: Box<[usize]>,
    pub icu4c: Option<DivergentOutput>,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct TestValue<'s> {
    pub input: &'s str,
    pub output: Output,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct TestCase<'s> {
    pub langid: &'s str,
    #[serde(borrow)]
    pub values: Box<[TestValue<'s>]>
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct TestData<'s>(
    #[serde(borrow)]
    pub Box<[TestCase<'s>]>
);
