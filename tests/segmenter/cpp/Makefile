CXX := c++

# Custom ICU
ICU_PATH:= /Users/zibi/projects/icu/icu4c/source
PKG := `pkg-config --libs --cflags jsoncpp` -I$(ICU_PATH)/i18n -I$(ICU_PATH)/common -L$(ICU_PATH)/lib -licui18n -licuuc -licudata

all: segmenter

segmenter: segmenter.cpp
	$(CXX) -std=c++20 -O3 -o segmenter segmenter.cpp $(ICU_PATH)/lib/libicui18n.a $(PKG)

clean:
	rm -f  segmenter
