Get data with:

```
cargo run --features bin,experimental -p icu_datagen -- --format blob --all-keys --all-locales --out ~/projects/icu-perf/data/icu4x-1.0.postcard --cldr-tag 41.0.0 --icuexport-tag release-71-1 --overwrite
```

```
cargo run --features bin,experimental -p icu_datagen -- --format mod --all-keys --all-locales --out ~/projects/icu-perf/data/icu4x-1.0.rs --cldr-tag 41.0.0 --icuexport-tag release-71-1 --overwrite
```

get tailored data for postcard with:

```
cargo run --features bin,experimental -p icu_datagen -- --format blob --keys-for-bin ~/projects/icu-perf/tests/*/target/release/examples/minimal --all-locales --out ~/projects/icu-perf/data/icu4x-1.0-*.postcard --cldr-tag 41.0.0 --icuexport-tag release-71-1 --overwrite
```
