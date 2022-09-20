#[cfg(feature = "icu4c")]
pub mod icu4c;

#[cfg(feature = "icu4x")]
pub mod icu4x;

fn main() {
    #[cfg(feature = "icu4c")]
    {
        let dtf = icu4c::DateTimeFormatter::new();
        let result = dtf.format();
        println!("ICU4C: {}", result);
    }

    #[cfg(feature = "icu4x")]
    {
        let dtf = icu4x::DateTimeFormatter::new();
        let result = dtf.format();
        println!("ICU4X: {}", result);
    }
}
