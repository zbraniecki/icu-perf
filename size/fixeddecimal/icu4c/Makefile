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
		-stdlib=libc++ \
		-licui18n -licuuc -licudata \
		-lstdc++ -lm -ldl -lpthread \
		-I$(ICU4C_INCLUDE) \
		-L$(ICU4C_LIB) \
		-Wl,--gc-sections

strip.elf: main.c
	$(CLANG) -static main.c -o $@ \
		-flto=thin -fuse-ld=$(LLD) \
		-stdlib=libc++ \
		-licui18n -licuuc -licudata \
		-lstdc++ -lm -ldl -lpthread \
		-I$(ICU4C_INCLUDE) \
		-L$(ICU4C_LIB) \
		-Wl,--gc-sections \
		-Wl,--strip-all

stub_debug.elf: main.c
	$(CLANG) -static main.c -o $@ \
		-flto=thin -fuse-ld=$(LLD) \
		-stdlib=libc++ \
		-licui18n -licuuc -licudata \
		-lstdc++ -lm -ldl -lpthread \
		-I$(ICU4C_INCLUDE) \
		-L$(ICU4C_STUBDATA_LIB) \
		-L$(ICU4C_LIB) \
		-Wl,--gc-sections

stub_strip.elf: main.c
	$(CLANG) -static main.c -o $@ \
		-flto=thin -fuse-ld=$(LLD) \
		-stdlib=libc++ \
		-licui18n -licuuc -licudata \
		-lstdc++ -lm -ldl -lpthread \
		-I$(ICU4C_INCLUDE) \
		-L$(ICU4C_STUBDATA_LIB) \
		-L$(ICU4C_LIB) \
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
