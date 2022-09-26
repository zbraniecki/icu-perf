pub fn get_data() -> TestData<'static> {
    TestData(
        Box::new([
            LocaleTestData {
                langid: Box::new(["en"]),
                values: Box::new([
                    TestCase {
                        input: 27832853,
                        style: ("None", "None"),
                        output: "",
                    },
                    TestCase {
                        input: 15932853,
                        style: ("None", "Short"),
                        output: "11:33 AM",
                    },
                    TestCase {
                        input: 18932853,
                        style: ("None", "Medium"),
                        output: "7:33:00 PM",
                    },
                    TestCase {
                        input: 18932853,
                        style: ("Short", "None"),
                        output: "12/30/05",
                    },
                    TestCase {
                        input: 16282853,
                        style: ("Medium", "None"),
                        output: "Dec 16, 2000",
                    },
                    TestCase {
                        input: 19012153,
                        style: ("Short", "Medium"),
                        output: "2/23/06, 9:13:00 PM",
                    },
                    TestCase {
                        input: 33102153,
                        style: ("Medium", "Short"),
                        output: "Dec 8, 2032, 2:33 PM",
                    },
                    TestCase {
                        input: 39100853,
                        style: ("Short", "Short"),
                        output: "5/5/44, 8:53 AM",
                    },
                    TestCase {
                        input: 31923823,
                        style: ("Medium", "Medium"),
                        output: "Sep 12, 2030, 7:43:00 AM",
                    },
                    TestCase {
                        input: 27114532,
                        style: ("Long", "Medium"),
                        output: "July 21, 2021 at 12:52:00 PM",
                    },
                ])
            },
        ])
    )
}

