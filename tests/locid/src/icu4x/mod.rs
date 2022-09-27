pub struct LanguageIdentifier {}

impl LanguageIdentifier {
    pub fn new() -> Self {
        Self {}
    }

    pub fn canonicalize(input: &str) -> String {
        icu_locid::LanguageIdentifier::canonicalize(input).unwrap()
    }
}
