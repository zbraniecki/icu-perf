#[cfg(feature = "icu4c")]
use icu_perf_test_segmenter::icu4c;

#[cfg(feature = "icu4x")]
use icu_perf_test_segmenter::icu4x;

fn main() {
    let langid = "en";
    let value = "Unter anderem war die Tatwaffe, eine Luger-Pistole, am Tag nach dem Mord in die Hände des Verfassungsschutzes gelangt, wo sie anschließend für 15 Jahre in einem Tresor versteckt und dies absichtlich vor den Strafverfolgungsbehörden verheimlicht wurde";

    #[cfg(feature = "icu4c")]
    {
        let seg = icu4c::Segmenter::new(langid, value, false);
        let result: Vec<_> = seg.collect();
        println!("ICU4C: {:?}", result);
    }

    #[cfg(feature = "icu4x")]
    {
        #[cfg(feature = "icu4x-static")]
        {
            let provider = icu4x::WordSegmenter::get_static_provider();
            let seg = icu4x::WordSegmenter::new_auto_static(&provider);
            let result: Vec<_> = seg.segment(value).collect();
            println!("ICU4X (static): {:?}", result);
        }

        #[cfg(feature = "icu4x-baked")]
        {
            let provider = icu4x::WordSegmenter::get_baked_provider();
            let seg = icu4x::WordSegmenter::new_auto_baked(&provider);
            let result: Vec<_> = seg.segment(value).collect();
            println!("ICU4X (baked): {:?}", result);
        }
    }
}
