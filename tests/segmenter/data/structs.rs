use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Auto,
    Dictionary,
    Lstm,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Dictionary => write!(f, "dictionary"),
            Self::Lstm => write!(f, "lstm"),
        }
    }
}

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
    pub ja_zh: bool,
    pub modes: Vec<Mode>,
    #[serde(borrow)]
    pub values: Box<[TestValue<'s>]>
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct TestData<'s>(
    #[serde(borrow)]
    pub Box<[TestCase<'s>]>
);
