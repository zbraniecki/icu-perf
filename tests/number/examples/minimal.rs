use icu_perf_test_number::icu4x;
use icu_locid::langid;
use icu_decimal::options::{FixedDecimalFormatterOptions, GroupingStrategy};

fn main() {
    let value = 27832853.0;
    let langid = langid!("en");
    let mut options: FixedDecimalFormatterOptions = Default::default();
    options.grouping_strategy = GroupingStrategy::Auto;

    let provider = icu4x::NumberFormatter::get_static_provider();
    let nf = icu4x::NumberFormatter::new_static(&provider, &langid, options);
    let _ = nf.format(value);
}
