#include <unicode/locid.h>
#include <unicode/utypes.h>
#include <unicode/datefmt.h>
#include <unicode/bytestream.h>
#include <stdio.h>
#include <chrono>
#include <vector>
#include <cstring>

static void show(void);

typedef std::tuple<UDate, const char*> value;

void show(void)
{
	std::vector<value> values = {
	  value(27832853, "Dec 2, 2022, 8:53:00 AM"),
	  value(15932853, "Apr 17, 2000, 11:33:00 AM"),
	  value(15932853, "Apr 17, 2000, 11:33:00 AM"),
	  value(15932853, "Apr 17, 2000, 11:33:00 AM"),
	  value(15932853, "Apr 17, 2000, 11:33:00 AM"),
	  value(27832853, "Dec 2, 2022, 8:53:00 AM"),
	  value(15932853, "Apr 17, 2000, 11:33:00 AM"),
	  value(15932853, "Apr 17, 2000, 11:33:00 AM"),
	  value(15932853, "Apr 17, 2000, 11:33:00 AM"),
	  value(15932853, "Apr 17, 2000, 11:33:00 AM"),
	};

	auto dtf = icu::DateFormat::createDateTimeInstance(
		icu::DateFormat::EStyle::MEDIUM,
		icu::DateFormat::EStyle::MEDIUM,
		"en"
	);
	icu::UnicodeString myString;
	dtf->format( std::get<0>(values[0]) * 60 * 1000, myString );

	std::string utf8;
	myString.toUTF8String(utf8);
	printf("date: %s\n", utf8.c_str());
}


int main() {
  show();
  return 0;
}

