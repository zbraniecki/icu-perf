using EStyle = icu_72::DateFormat::EStyle;
typedef std::tuple<UDate, const char*, EStyle, EStyle, const char*> value;

std::vector<value> values = {
  value(27832853.0, "en", EStyle::NONE, EStyle::NONE, "Dec 2, 2022, 8:53:00 AM"),
  value(15932853.0, "en", EStyle::NONE, EStyle::SHORT, "Apr 17, 2000, 11:33:00 AM"),
  value(15932853.0, "en", EStyle::NONE, EStyle::MEDIUM, "Apr 17, 2000, 11:33:00 AM"),
  value(15932853.0, "en", EStyle::NONE, EStyle::LONG, "Apr 17, 2000, 11:33:00 AM"),
  value(15932853.0, "en", EStyle::NONE, EStyle::FULL, "Apr 17, 2000, 11:33:00 AM"),
};
