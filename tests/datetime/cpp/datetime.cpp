#include <unicode/locid.h>
#include <unicode/utypes.h>
#include <unicode/datefmt.h>
#include <unicode/bytestream.h>
#include <stdio.h>
#include <chrono>
#include <vector>
#include <cstring>
#include "../data/datetime.cpp"

static void show(void);

void show(void)
{
	auto dtf = icu::DateFormat::createDateTimeInstance(
		icu::DateFormat::EStyle::MEDIUM,
		icu::DateFormat::EStyle::MEDIUM,
		"en"
	);
	icu::UnicodeString myString;
	dtf->format(
	  std::get<0>(std::get<1>(data[0])[0]) * 60 * 1000,
	  myString );

	std::string utf8;
	myString.toUTF8String(utf8);
	printf("date: %s\n", utf8.c_str());
}


int main() {
  show();
  return 0;
}

