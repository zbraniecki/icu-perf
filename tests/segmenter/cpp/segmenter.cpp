#include <unicode/locid.h>
#include <unicode/rbbi.h>
#include <unicode/brkiter.h>
#include <unicode/utypes.h>
#include <unicode/bytestream.h>
#include <stdio.h>
#include <chrono>
#include <vector>
#include <cstring>
#include <fstream>
#include <iostream>
#include <sstream>
// #include "../data/datetime.cpp"

using namespace std;

static void show(void);

string format_vector(const vector<int32_t>& vec) {
  stringstream sstr;

  auto iter = vec.begin();
  if (iter == vec.end()) {
    return string();
  }

  sstr << *iter;
  while (++iter != vec.end()) {
    sstr << ", " << *iter;
  }

  return sstr.str();
}

void show(void)
{
  std::string strUtf8 = "Hello World";
  const char* locale = "en";

  const auto strUtf16 = icu::UnicodeString::fromUTF8(strUtf8);
  UErrorCode status = U_ZERO_ERROR;
  icu::BreakIterator* iter = icu::BreakIterator::createLineInstance(icu::Locale(locale), status);
  iter->setText(strUtf16);

  int32_t nextPos = UBRK_DONE;
  vector<int32_t> breaks;
  while((nextPos = iter->next()) != UBRK_DONE) {
    breaks.push_back(nextPos);
  }
  cout << "Line break opportunities: [" << format_vector(breaks) << "]" << endl;
}


int main() {
  show();
  return 0;
}

