pub fn get_data() -> TestData<'static> {
    TestData(
        Box::new([
            LocaleTestData {
                langid: Box::new(["en"]),
                values: Box::new([
                    TestCase {
                        input: 27832853,
                        style: ("None", "None"),
                        output: "Dec 2, 2022, 8:53:00 AM",
                    },
                    TestCase {
                        input: 15932853,
                        style: ("None", "Short"),
                        output: "Apr 17, 2000, 11:33:00 AM",
                    },
                    TestCase {
                        input: 15932853,
                        style: ("None", "Medium"),
                        output: "Apr 17, 2000, 11:33:00 AM",
                    },
                    TestCase {
                        input: 15932853,
                        style: ("None", "Long"),
                        output: "Apr 17, 2000, 11:33:00 AM",
                    },
                    TestCase {
                        input: 15932853,
                        style: ("None", "Full"),
                        output: "Apr 17, 2000, 11:33:00 AM",
                    },
                ])
            }
        ])
    )
}
