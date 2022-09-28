#

.DEFAULT_GOAL := test
.PHONY: build clean test

GCC := gcc
CLANG := clang-14
LLD := lld-14

ICU4C_LIB := ../../icu4c-opt-size/usr/lib
ICU4C_INCLUDE := ../../icu4c-opt-size/usr/include
ICU4C_STUBDATA_LIB := ../../icu4c-opt-size/build/stubdata

debug.elf: main.c
	$(CLANG) -static main.c -o $@ \
		-flto=thin -fuse-ld=$(LLD) \
		$(ICU4C_LIB)/libicui18n.a \
		$(ICU4C_LIB)/libicuuc.a \
		$(ICU4C_LIB)/libicudata.a \
		-stdlib=libc++ -lstdc++ -lm -ldl -lpthread \
		-I$(ICU4C_INCLUDE) \
		-Wl,--gc-sections

strip.elf: main.c
	$(CLANG) -static main.c -o $@ \
		-flto=thin -fuse-ld=$(LLD) \
		$(ICU4C_LIB)/libicui18n.a \
		$(ICU4C_LIB)/libicuuc.a \
		$(ICU4C_LIB)/libicudata.a \
		-stdlib=libc++ -lstdc++ -lm -ldl -lpthread \
		-I$(ICU4C_INCLUDE) \
		-Wl,--gc-sections \
		-Wl,--strip-all

stub_debug.elf: main.c
	$(CLANG) -static main.c -o $@ \
		-flto=thin -fuse-ld=$(LLD) \
		$(ICU4C_LIB)/libicui18n.a \
		$(ICU4C_LIB)/libicuuc.a \
		$(ICU4C_STUBDATA_LIB)/libicudata.a \
		-stdlib=libc++ -lstdc++ -lm -ldl -lpthread \
		-I$(ICU4C_INCLUDE) \
		-Wl,--gc-sections

stub_strip.elf: main.c
	$(CLANG) -static main.c -o $@ \
		-flto=thin -fuse-ld=$(LLD) \
		$(ICU4C_LIB)/libicui18n.a \
		$(ICU4C_LIB)/libicuuc.a \
		$(ICU4C_STUBDATA_LIB)/libicudata.a \
		-stdlib=libc++ -lstdc++ -lm -ldl -lpthread \
		-I$(ICU4C_INCLUDE) \
		-Wl,--gc-sections \
		-Wl,--strip-all

ALL_ELF=debug.elf strip.elf stub_debug.elf stub_strip.elf

build: $(ALL_ELF)

clean:
	rm -f $(ALL_ELF)

test: build
	./debug.elf
	./strip.elf
	ls -l
