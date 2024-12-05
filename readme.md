Get data with:

```
cargo run --release -p icu4x-datagen --all-features -- --format blob --markers all --locales modern --out ~/projects/icu-perf/data/icu4x-2.0-alpha.postcard --overwrite
```

```
cargo run --release -p icu4x-datagen --all-features -- --format mod --markers all --locales modern --out ~/projects/icu-perf/data/icu4x-2.0-alpha.rs --overwrite
```

get tailored data for postcard with:

```
cargo run --features bin -p icu_datagen -- --format blob --keys-for-bin ~/projects/icu-perf/tests/*/target/release/examples/minimal --locales modern --out ~/projects/icu-perf/data/icu4x-2.0-alpha-*.postcard --overwrite
```
