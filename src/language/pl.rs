use crate::{Currency, FractionalUnit, Language, Texts, WordsMapping};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref PL: Language = Language {
        currency: Currency {
            // name: "Polski Złoty",
            singular: "Złoty",
            plural: "Złotych",
            // symbol: "zł",
            fractional_unit: FractionalUnit {
                // name: "Grosz",
                singular: "Grosz",
                plural: "Groszy",
                // symbol: "gr",
            },
        },
        texts: Texts {
            and: "I",
            minus: "Minus",
            // only: "Tylko",
            point: "Przecinek",
        },
        word_mapping: vec![
            WordsMapping {number: 1_000_000_000_000_000_000, value: "Trylion",          plural: Some("Tryliony") }, // Next number would be out of range
            WordsMapping {number: 1_000_000_000_000_000,     value: "Biliard",          plural: Some("Biliardów")},
            WordsMapping {number: 1_000_000_000_000,         value: "Bilion",           plural: Some("Bilionów") },
            WordsMapping {number: 1_000_000_000,             value: "Miliard",          plural: Some("Miliardów")},
            WordsMapping {number: 1_000_000,                 value: "Milion",           plural: Some("Milionów") },
            WordsMapping {number: 1_000,                     value: "Tysiąc",           plural: Some("Tysięcy")  },
            WordsMapping {number: 900,                       value: "Dziewięćset",      plural: None             },
            WordsMapping {number: 800,                       value: "Osiemset",         plural: None             },
            WordsMapping {number: 700,                       value: "Siedemset",        plural: None             },
            WordsMapping {number: 600,                       value: "Sześćset",         plural: None             },
            WordsMapping {number: 500,                       value: "Pięćset",          plural: None             },
            WordsMapping {number: 400,                       value: "Czterysta",        plural: None             },
            WordsMapping {number: 300,                       value: "Trzysta",          plural: None             },
            WordsMapping {number: 200,                       value: "Dwieście",         plural: None             },
            WordsMapping {number: 100,                       value: "Sto",              plural: None             },
            WordsMapping {number: 90,                        value: "Dziewięćdziesiąt", plural: None             },
            WordsMapping {number: 80,                        value: "Osiemdziesiąt",    plural: None             },
            WordsMapping {number: 70,                        value: "Siedemdziesiąt",   plural: None             },
            WordsMapping {number: 60,                        value: "Sześćdziesiąt",    plural: None             },
            WordsMapping {number: 50,                        value: "Pięćdziesiąt",     plural: None             },
            WordsMapping {number: 40,                        value: "Czterdzieści",     plural: None             },
            WordsMapping {number: 30,                        value: "Trzydzieści",      plural: None             },
            WordsMapping {number: 20,                        value: "Dwadzieścia",      plural: None             },
            WordsMapping {number: 19,                        value: "Dziewiętnaście",   plural: None             },
            WordsMapping {number: 18,                        value: "Osiemnaście",      plural: None             },
            WordsMapping {number: 17,                        value: "Siedemnaście",     plural: None             },
            WordsMapping {number: 16,                        value: "Szesnaście",       plural: None             },
            WordsMapping {number: 15,                        value: "Piętnaście",       plural: None             },
            WordsMapping {number: 14,                        value: "Czternaście",      plural: None             },
            WordsMapping {number: 13,                        value: "Trzynaście",       plural: None             },
            WordsMapping {number: 12,                        value: "Dwanaście",        plural: None             },
            WordsMapping {number: 11,                        value: "Jedynaście",       plural: None             },
            WordsMapping {number: 10,                        value: "Dziesięć",         plural: None             },
            WordsMapping {number: 9,                         value: "Dziewięć",         plural: None             },
            WordsMapping {number: 8,                         value: "Osiem",            plural: None             },
            WordsMapping {number: 7,                         value: "Siedem",           plural: None             },
            WordsMapping {number: 6,                         value: "Sześć",            plural: None             },
            WordsMapping {number: 5,                         value: "Pięć",             plural: None             },
            WordsMapping {number: 4,                         value: "Cztery",           plural: None             },
            WordsMapping {number: 3,                         value: "Trzy",             plural: None             },
            WordsMapping {number: 2,                         value: "Dwa",              plural: None             },
            WordsMapping {number: 1,                         value: "Jeden",            plural: None             },
            WordsMapping {number: 0,                         value: "Zero",             plural: None             },
        ],
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(PL.convert(0, false), "Zero");
        assert_eq!(PL.convert(1, false), "Jeden");
        assert_eq!(PL.convert(10, false), "Dziesięć");
        assert_eq!(PL.convert(11, false), "Jedynaście");
        assert_eq!(PL.convert(99, false), "Dziewięćdziesiąt Dziewięć");
        assert_eq!(
            PL.convert(999, false),
            "Dziewięćset Dziewięćdziesiąt Dziewięć"
        );
    }

    #[test]
    fn big_numbers() {
        assert_eq!(
            PL.convert(69_420, false),
            "Sześćdziesiąt Dziewięć Tysięcy Czterysta Dwadzieścia"
        );
        assert_eq!(
            PL.convert(9_999_999_999_999_999 as i64, false),
            "Dziewięć Biliardów Dziewięćset Dziewięćdziesiąt Dziewięć Bilionów Dziewięćset Dziewięćdziesiąt Dziewięć Miliardów Dziewięćset Dziewięćdziesiąt Dziewięć Milionów Dziewięćset Dziewięćdziesiąt Dziewięć Tysięcy Dziewięćset Dziewięćdziesiąt Dziewięć"
        );
        assert_eq!(
            PL.convert(1_457_923_567_901_234_567 as i64, false),
            "Trylion Czterysta Pięćdziesiąt Siedem Biliardów Dziewięćset Dwadzieścia Trzy Bilionów Pięćset Sześćdziesiąt Siedem Miliardów Dziewięćset Jeden Milionów Dwieście Trzydzieści Cztery Tysięcy Pięćset Sześćdziesiąt Siedem"
        );
        assert_eq!(
            PL.convert(2_000_000_000_000_000_000 as i64, false),
            "Dwa Tryliony"
        );
    }

    #[test]
    fn negative_numbers() {
        assert_eq!(PL.convert(-1, false), "Minus Jeden");
        assert_eq!(PL.convert(-10, false), "Minus Dziesięć");
        assert_eq!(PL.convert(-11, false), "Minus Jedynaście");
        assert_eq!(PL.convert(-99, false), "Minus Dziewięćdziesiąt Dziewięć");
        assert_eq!(
            PL.convert(-999, false),
            "Minus Dziewięćset Dziewięćdziesiąt Dziewięć"
        );
    }

    #[test]
    fn big_negative_numbers() {
        assert_eq!(
            PL.convert(-69_420, false),
            "Minus Sześćdziesiąt Dziewięć Tysięcy Czterysta Dwadzieścia"
        );
        assert_eq!(
            PL.convert(-9_999_999_999_999_999 as i64, false),
            "Minus Dziewięć Biliardów Dziewięćset Dziewięćdziesiąt Dziewięć Bilionów Dziewięćset Dziewięćdziesiąt Dziewięć Miliardów Dziewięćset Dziewięćdziesiąt Dziewięć Milionów Dziewięćset Dziewięćdziesiąt Dziewięć Tysięcy Dziewięćset Dziewięćdziesiąt Dziewięć"
        );
        assert_eq!(
            PL.convert(-1_457_923_567_901_234_567 as i64, false),
            "Minus Trylion Czterysta Pięćdziesiąt Siedem Biliardów Dziewięćset Dwadzieścia Trzy Bilionów Pięćset Sześćdziesiąt Siedem Miliardów Dziewięćset Jeden Milionów Dwieście Trzydzieści Cztery Tysięcy Pięćset Sześćdziesiąt Siedem"
        );
        assert_eq!(
            PL.convert(-2_000_000_000_000_000_000 as i64, false),
            "Minus Dwa Tryliony"
        );
    }

    #[test]
    fn simple_floating_point_numbers() {
        assert_eq!(PL.convert(0.01, false), "Zero Przecinek Jeden");
        assert_eq!(
            PL.convert(1.99, false),
            "Jeden Przecinek Dziewięćdziesiąt Dziewięć"
        );
        assert_eq!(PL.convert(10.10, false), "Dziesięć Przecinek Dziesięć");
        assert_eq!(PL.convert(11.11, false), "Jedynaście Przecinek Jedynaście");
        assert_eq!(
            PL.convert(99.99, false),
            "Dziewięćdziesiąt Dziewięć Przecinek Dziewięćdziesiąt Dziewięć"
        );
        assert_eq!(
            PL.convert(999.01, false),
            "Dziewięćset Dziewięćdziesiąt Dziewięć Przecinek Jeden"
        );
    }

    #[test]
    fn big_floating_point_numbers() {
        assert_eq!(
            PL.convert(69_420.69, false),
            "Sześćdziesiąt Dziewięć Tysięcy Czterysta Dwadzieścia Przecinek Sześćdziesiąt Dziewięć"
        );
        assert_eq!(
            PL.convert(9_999_999_999_999.99, false),
            "Dziewięć Bilionów Dziewięćset Dziewięćdziesiąt Dziewięć Miliardów Dziewięćset Dziewięćdziesiąt Dziewięć Milionów Dziewięćset Dziewięćdziesiąt Dziewięć Tysięcy Dziewięćset Dziewięćdziesiąt Dziewięć Przecinek Dziewięćdziesiąt Dziewięć"
        );
        assert_eq!(
            PL.convert(1_457_923_567_901.12, false),
            "Bilion Czterysta Pięćdziesiąt Siedem Miliardów Dziewięćset Dwadzieścia Trzy Milionów Pięćset Sześćdziesiąt Siedem Tysięcy Dziewięćset Jeden Przecinek Dwanaście"
        );
        assert_eq!(
            PL.convert(2_000_000_000_000.20 as f64, false),
            "Dwa Bilionów Przecinek Dwadzieścia"
        );
    }

    #[test]
    fn simple_money() {
        assert_eq!(PL.convert(0, true), "Zero Złotych");
        assert_eq!(PL.convert(1, true), "Jeden Złoty");
        assert_eq!(PL.convert(10, true), "Dziesięć Złotych");
        assert_eq!(PL.convert(11, true), "Jedynaście Złotych");
        assert_eq!(PL.convert(99, true), "Dziewięćdziesiąt Dziewięć Złotych");
        assert_eq!(
            PL.convert(999, true),
            "Dziewięćset Dziewięćdziesiąt Dziewięć Złotych"
        );
    }

    #[test]
    fn floating_point_money() {
        assert_eq!(PL.convert(0.01, true), "Zero Złotych I Jeden Grosz");
        assert_eq!(
            PL.convert(1.99, true),
            "Jeden Złoty I Dziewięćdziesiąt Dziewięć Groszy"
        );
        assert_eq!(
            PL.convert(10.10, true),
            "Dziesięć Złotych I Dziesięć Groszy"
        );
        assert_eq!(
            PL.convert(11.11, true),
            "Jedynaście Złotych I Jedynaście Groszy"
        );
        assert_eq!(
            PL.convert(99.99, true),
            "Dziewięćdziesiąt Dziewięć Złotych I Dziewięćdziesiąt Dziewięć Groszy"
        );
        assert_eq!(
            PL.convert(999.01, true),
            "Dziewięćset Dziewięćdziesiąt Dziewięć Złotych I Jeden Grosz"
        );
    }

    #[test]
    fn big_money_numbers() {
        assert_eq!(
            PL.convert(69_420, true),
            "Sześćdziesiąt Dziewięć Tysięcy Czterysta Dwadzieścia Złotych"
        );
        assert_eq!(
            PL.convert(9_999_999_999_999_999 as i64, true),
            "Dziewięć Biliardów Dziewięćset Dziewięćdziesiąt Dziewięć Bilionów Dziewięćset Dziewięćdziesiąt Dziewięć Miliardów Dziewięćset Dziewięćdziesiąt Dziewięć Milionów Dziewięćset Dziewięćdziesiąt Dziewięć Tysięcy Dziewięćset Dziewięćdziesiąt Dziewięć Złotych"
        );
        assert_eq!(
            PL.convert(1_457_923_567_901_234_567 as i64, true),
            "Trylion Czterysta Pięćdziesiąt Siedem Biliardów Dziewięćset Dwadzieścia Trzy Bilionów Pięćset Sześćdziesiąt Siedem Miliardów Dziewięćset Jeden Milionów Dwieście Trzydzieści Cztery Tysięcy Pięćset Sześćdziesiąt Siedem Złotych"
        );
        assert_eq!(
            PL.convert(2_000_000_000_000_000_000 as i64, true),
            "Dwa Tryliony Złotych"
        );
    }

    #[test]
    fn big_floating_point_money_numbers() {
        assert_eq!(
            PL.convert(69_420.69, true),
            "Sześćdziesiąt Dziewięć Tysięcy Czterysta Dwadzieścia Złotych I Sześćdziesiąt Dziewięć Groszy"
        );
        assert_eq!(
            PL.convert(9_999_999_999_999.99, true),
            "Dziewięć Bilionów Dziewięćset Dziewięćdziesiąt Dziewięć Miliardów Dziewięćset Dziewięćdziesiąt Dziewięć Milionów Dziewięćset Dziewięćdziesiąt Dziewięć Tysięcy Dziewięćset Dziewięćdziesiąt Dziewięć Złotych I Dziewięćdziesiąt Dziewięć Groszy"
        );
        assert_eq!(
            PL.convert(1_457_923_567_901.12, true),
            "Bilion Czterysta Pięćdziesiąt Siedem Miliardów Dziewięćset Dwadzieścia Trzy Milionów Pięćset Sześćdziesiąt Siedem Tysięcy Dziewięćset Jeden Złotych I Dwanaście Groszy"
        );
        assert_eq!(
            PL.convert(2_000_000_000_000.20 as f64, true),
            "Dwa Bilionów Złotych I Dwadzieścia Groszy"
        );
    }
}
