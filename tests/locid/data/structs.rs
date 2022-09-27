use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct TestData<'s>(
    #[serde(borrow)]
    pub Box<[(&'s str, &'s str)]>
);
