Get data with:

```
cargo run --features bin -p icu_datagen -- --format blob --all-keys --locales modern --out ~/projects/icu-perf/data/icu4x-1.5.postcard --overwrite
```

```
cargo run --features bin -p icu_datagen -- --format mod --all-keys --locales modern --out ~/projects/icu-perf/data/icu4x-1.5.rs --overwrite
```

get tailored data for postcard with:

```
cargo run --features bin -p icu_datagen -- --format blob --keys-for-bin ~/projects/icu-perf/tests/*/target/release/examples/minimal --locales modern --out ~/projects/icu-perf/data/icu4x-1.5-*.postcard --overwrite
```
