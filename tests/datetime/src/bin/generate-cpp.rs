use serde;

mod data {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/structs.rs"));
}

const TEST_DATA: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/datetime.json"));

fn get_estyle(input: &str) -> &'static str {
    match input {
        "None" => "EStyle::NONE",
        "Short" => "EStyle::SHORT",
        "Medium" => "EStyle::MEDIUM",
        "Long" => "EStyle::LONG",
        "Full" => "EStyle::FULL",
        _ => unreachable!(),
    }
}

fn main() {
//     let tests: data::TestData = serde_json::from_str(TEST_DATA).expect("Failed to parse test JSON");
//
//     let mut result = String::from("using EStyle = icu_72::DateFormat::EStyle;\n");
//     result.push_str(
//         r#"
// typedef std::tuple<UDate, EStyle, EStyle, const char*> testCase;
// typedef std::vector<const char*> langids;
// typedef std::vector<testCase> values;
// typedef std::tuple<langids, values> localeTestData;
// typedef std::vector<localeTestData> testData;
//
// "#,
//     );
//
//     result.push_str("testData data = {\n");
//     for test in tests.0.iter() {
//         result.push_str("  localeTestData(\n");
//         result.push_str(&format!(
//             "    {{{0}}},\n",
//             test.langid
//                 .iter()
//                 .map(|l| format!("\"{0}\"", l))
//                 .collect::<Vec<_>>()
//                 .join(",")
//         ));
//         result.push_str("    {\n");
//         for case in test.values.iter() {
//             result.push_str(&format!(
//                 "      testCase({0}, {1}, {2}, \"{3}\"),\n",
//                 case.input,
//                 get_estyle(case.style.0),
//                 get_estyle(case.style.1),
//                 case.output
//             ));
//         }
//         result.push_str("    }\n");
//         result.push_str("  ),\n");
//     }
//     result.push_str("};");
//     //     let mut result = String::from(r#"pub fn get_data() -> TestData<'static> {
//     //     TestData(
//     //         Box::new(["#);
//     //     for test in tests.0.iter() {
//     //         result.push_str(r#"
//     //             LocaleTestData {
//     //                 langid: Box::new(["#);
//     //         let langids = "\"en\"";
//     //         result.push_str(langids);
//     //         result.push_str(r#"]),
//     //                 values: Box::new(["#);
//     //         for case in test.values.iter() {
//     //             result.push_str(&format!(r#"
//     //                     TestCase {{
//     //                         input: {0},
//     //                         style: ("{1}", "{2}"),
//     //                         output: "{3}",
//     //                     }},"#, case.input, case.style.0, case.style.1, case.output));
//     //         }
//     //         result.push_str(r#"
//     //                 ])
//     //             },"#);
//     //     }
//     //     result.push_str(r#"
//     //         ])
//     //     )
//     // }
//     // "#);
//
//     println!("{}", result);
}
