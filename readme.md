Get data with:

```
cargo run --features bin -p icu_datagen -- --format blob --keys experimental-all --locales modern --out ~/projects/icu-perf/data/icu4x-1.2.postcard --cldr-tag 42.0.0 --icuexport-tag icu4x/2023-02-24/72.x --overwrite
```

```
cargo run --features bin -p icu_datagen -- --format mod --keys experimental-all --locales modern --out ~/projects/icu-perf/data/icu4x-1.2.rs --cldr-tag 42.0.0 --icuexport-tag icu4x/2023-02-24/72.x --overwrite
```

get tailored data for postcard with:

```
cargo run --features bin -p icu_datagen -- --format blob --keys-for-bin ~/projects/icu-perf/tests/*/target/release/examples/minimal --locales modern --out ~/projects/icu-perf/data/icu4x-1.2-*.postcard --cldr-tag 42.0.0 --icuexport-tag icu4x/2023-02-24/72.x --overwrite
```
