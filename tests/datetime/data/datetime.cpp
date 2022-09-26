using EStyle = icu_72::DateFormat::EStyle;

typedef std::tuple<UDate, EStyle, EStyle, const char*> testCase;
typedef std::vector<const char*> langids;
typedef std::vector<testCase> values;
typedef std::tuple<langids, values> localeTestData;
typedef std::vector<localeTestData> testData;

testData data = {
  localeTestData(
    {"en"},
    {
      testCase(27832853, EStyle::NONE, EStyle::NONE, ""),
      testCase(15932853, EStyle::NONE, EStyle::SHORT, "11:33 AM"),
      testCase(18932853, EStyle::NONE, EStyle::MEDIUM, "7:33:00 PM"),
      testCase(18932853, EStyle::SHORT, EStyle::NONE, "12/30/05"),
      testCase(16282853, EStyle::MEDIUM, EStyle::NONE, "Dec 16, 2000"),
      testCase(19012153, EStyle::SHORT, EStyle::MEDIUM, "2/23/06, 9:13:00 PM"),
      testCase(33102153, EStyle::MEDIUM, EStyle::SHORT, "Dec 8, 2032, 2:33 PM"),
      testCase(39100853, EStyle::SHORT, EStyle::SHORT, "5/5/44, 8:53 AM"),
      testCase(31923823, EStyle::MEDIUM, EStyle::MEDIUM, "Sep 12, 2030, 7:43:00 AM"),
      testCase(27114532, EStyle::LONG, EStyle::MEDIUM, "July 21, 2021 at 12:52:00 PM"),
    }
  ),
};
