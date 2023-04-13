Get data with:

```
cargo run --features bin -p icu_datagen -- --format blob --all-keys --locales modern --out ~/projects/icu-perf/data/icu4x-1.2.postcard --cldr-tag 43.0.0 --icuexport-tag icu4x/2023-03-22a/72.x --overwrite
```

```
cargo run --features bin -p icu_datagen -- --format mod --all-keys --locales modern --out ~/projects/icu-perf/data/icu4x-1.2.rs --cldr-tag 43.0.0 --icuexport-tag icu4x/2023-03-22a/72.x --overwrite
```

get tailored data for postcard with:

```
cargo run --features bin -p icu_datagen -- --format blob --keys-for-bin ~/projects/icu-perf/tests/*/target/release/examples/minimal --locales modern --out ~/projects/icu-perf/data/icu4x-1.2-*.postcard --cldr-tag 43.0.0 --icuexport-tag icu4x/2023-03-22a/72.x --overwrite
```
