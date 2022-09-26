use serde;

mod data {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/structs.rs"));
}

const TEST_DATA: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/datetime.json"));

fn main() {
    let tests: data::TestData = serde_json::from_str(TEST_DATA).expect("Failed to parse test JSON");

    let mut result = String::from(
        r#"pub fn get_data() -> TestData<'static> {
    TestData(
        Box::new(["#,
    );
    for test in tests.0.iter() {
        result.push_str(
            r#"
            LocaleTestData {
                langid: Box::new(["#,
        );
        let langids = "\"en\"";
        result.push_str(langids);
        result.push_str(
            r#"]),
                values: Box::new(["#,
        );
        for case in test.values.iter() {
            result.push_str(&format!(
                r#"
                    TestCase {{
                        input: {0},
                        style: ("{1}", "{2}"),
                        output: "{3}",
                    }},"#,
                case.input, case.style.0, case.style.1, case.output
            ));
        }
        result.push_str(
            r#"
                ])
            },"#,
        );
    }
    result.push_str(
        r#"
        ])
    )
}
"#,
    );

    println!("{}", result);
}
