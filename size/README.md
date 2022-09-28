Code Size Benchmarks
====================

Install dependencies:

```bash
$ sudo apt-get install llvm-14 lld-14 libc++-14-dev libc++abi-14-dev
```

I use llvm-14 because that is the version ICU4X is pinned at.

Build ICU4C with size optimized and LTO enabled:

```bash
# Start in this directory:
cd icu-perf/size

# The following build directory is in the gitignore:
mkdir -p icu4c-opt-size/build
cd icu4c-opt-size/build

# Run configure and make:
CC="clang-14" CXX="clang++-14" LDFLAGS="-stdlib=libc++ -flto" CFLAGS="-Os" CXXFLAGS="-stdlib=libc++ -Os" /path/to/runConfigureICU Linux/clang --prefix $PWD/../usr --enable-static --disable-shared
make -j6

# Install it to ../usr according to the prefix above
make install
```

Then you can build and run the examples:

```bash
cd icu-perf/size/fixeddecimal/icu4c
make
````

This produces 4 files:

- `debug.elf` is a statically linked, runnable executable with debug symbols
- `strip.elf` is a statically linked, runnable executable with debug symbol names removed for a smaller size (but almost impossible to debug or analyze)
- `stub_debug.elf` is a statically linked executable with the ICU stubdata; it doesn't run, but it lets you see the code size without data size
- `stub_strip.elf` is like `stub_debug.elf` but with symbol names removed (this is the smallest of the four files)
