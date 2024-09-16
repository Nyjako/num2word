use crate::{Currency, FractionalUnit, Language, Texts, WordsMapping};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref EN_US: Language = Language {
        currency: Currency {
            // name: "United States Dollar",
            singular: "Dollar",
            plural: "Dollars",
            // symbol: "$",
            fractional_unit: FractionalUnit {
                // name: "Cent",
                plural: "Cent",
                singular: "Cents",
                // symbol: "¢",
            },
        },
        texts: Texts {
            and: "and",
            minus: "minus",
            // only: "only",
            point: "point",
        },
        word_mapping: vec![
            WordsMapping {number: 1_000_000_000_000_000_000, value: "Quintillion",   plural: Some("Quintillions") },
            WordsMapping {number: 1_000_000_000_000_000,     value: "Quadrillion",   plural: Some("Quadrillions") },
            WordsMapping {number: 1_000_000_000_000,         value: "Trillion",      plural: Some("Trillions")    },
            WordsMapping {number: 1_000_000_000,             value: "Billion",       plural: Some("Billions")     },
            WordsMapping {number: 1_000_000,                 value: "Million",       plural: Some("Millions")     },
            WordsMapping {number: 1_000,                     value: "Thousand",      plural: Some("Thousands")    },
            WordsMapping {number: 100,                       value: "Hundred",       plural: None                 },
            WordsMapping {number: 90,                        value: "Ninety",        plural: None                 },
            WordsMapping {number: 80,                        value: "Eighty",        plural: None                 },
            WordsMapping {number: 70,                        value: "Seventy",       plural: None                 },
            WordsMapping {number: 60,                        value: "Sixty",         plural: None                 },
            WordsMapping {number: 50,                        value: "Fifty",         plural: None                 },
            WordsMapping {number: 40,                        value: "Forty",         plural: None                 },
            WordsMapping {number: 30,                        value: "Thirty",        plural: None                 },
            WordsMapping {number: 20,                        value: "Twenty",        plural: None                 },
            WordsMapping {number: 19,                        value: "Nineteen",      plural: None                 },
            WordsMapping {number: 18,                        value: "Eighteen",      plural: None                 },
            WordsMapping {number: 17,                        value: "Seventeen",     plural: None                 },
            WordsMapping {number: 16,                        value: "Sixteen",       plural: None                 },
            WordsMapping {number: 15,                        value: "Fifteen",       plural: None                 },
            WordsMapping {number: 14,                        value: "Fourteen",      plural: None                 },
            WordsMapping {number: 13,                        value: "Thirteen",      plural: None                 },
            WordsMapping {number: 12,                        value: "Twelve",        plural: None                 },
            WordsMapping {number: 11,                        value: "Eleven",        plural: None                 },
            WordsMapping {number: 10,                        value: "Ten",           plural: None                 },
            WordsMapping {number: 9,                         value: "Nine",          plural: None                 },
            WordsMapping {number: 8,                         value: "Eight",         plural: None                 },
            WordsMapping {number: 7,                         value: "Seven",         plural: None                 },
            WordsMapping {number: 6,                         value: "Six",           plural: None                 },
            WordsMapping {number: 5,                         value: "Five",          plural: None                 },
            WordsMapping {number: 4,                         value: "Four",          plural: None                 },
            WordsMapping {number: 3,                         value: "Three",         plural: None                 },
            WordsMapping {number: 2,                         value: "Two",           plural: None                 },
            WordsMapping {number: 1,                         value: "One",           plural: None                 },
            WordsMapping {number: 0,                         value: "Zero",          plural: None                 },
        ],
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(EN_US.convert(0, false), "Zero");
        assert_eq!(EN_US.convert(1, false), "One");
        assert_eq!(EN_US.convert(10, false), "Ten");
        assert_eq!(EN_US.convert(11, false), "Eleven");
        assert_eq!(EN_US.convert(99, false), "Ninety Nine");
        assert_eq!(EN_US.convert(999, false), "Nine Hundred Ninety Nine");
    }

    #[test]
    fn big_numbers() {
        assert_eq!(
            EN_US.convert(69_420, false),
            "Sixty Nine Thousands Four Hundred Twenty"
        );
        assert_eq!(
            EN_US.convert(9_999_999_999_999_999 as i64, false),
            "Nine Quadrillions Nine Hundred Ninety Nine Trillions Nine Hundred Ninety Nine Billions Nine Hundred Ninety Nine Millions Nine Hundred Ninety Nine Thousands Nine Hundred Ninety Nine"
        );
        assert_eq!(
            EN_US.convert(1_457_923_567_901_234_567 as i64, false),
            "Quintillion Four Hundred Fifty Seven Quadrillions Nine Hundred Twenty Three Trillions Five Hundred Sixty Seven Billions Nine Hundred One Millions Two Hundred Thirty Four Thousands Five Hundred Sixty Seven"
        );
        assert_eq!(
            EN_US.convert(2_000_000_000_000_000_000 as i64, false),
            "Two Quintillions"
        );
    }

    #[test]
    fn negative_numbers() {
        assert_eq!(EN_US.convert(-1, false), "minus One");
        assert_eq!(EN_US.convert(-10, false), "minus Ten");
        assert_eq!(EN_US.convert(-11, false), "minus Eleven");
        assert_eq!(EN_US.convert(-99, false), "minus Ninety Nine");
        assert_eq!(EN_US.convert(-999, false), "minus Nine Hundred Ninety Nine");
    }

    #[test]
    fn big_negative_numbers() {
        assert_eq!(
            EN_US.convert(-69_420, false),
            "minus Sixty Nine Thousands Four Hundred Twenty"
        );
        assert_eq!(
            EN_US.convert(-9_999_999_999_999_999 as i64, false),
            "minus Nine Quadrillions Nine Hundred Ninety Nine Trillions Nine Hundred Ninety Nine Billions Nine Hundred Ninety Nine Millions Nine Hundred Ninety Nine Thousands Nine Hundred Ninety Nine"
        );
        assert_eq!(
            EN_US.convert(-1_457_923_567_901_234_567 as i64, false),
            "minus Quintillion Four Hundred Fifty Seven Quadrillions Nine Hundred Twenty Three Trillions Five Hundred Sixty Seven Billions Nine Hundred One Millions Two Hundred Thirty Four Thousands Five Hundred Sixty Seven"
        );
        assert_eq!(
            EN_US.convert(-2_000_000_000_000_000_000 as i64, false),
            "minus Two Quintillions"
        );
    }

    #[test]
    fn simple_floating_point_numbers() {
        assert_eq!(EN_US.convert(0.01, false), "Zero point One");
        assert_eq!(EN_US.convert(1.99, false), "One point Ninety Nine");
        assert_eq!(EN_US.convert(10.10, false), "Ten point Ten");
        assert_eq!(EN_US.convert(11.11, false), "Eleven point Eleven");
        assert_eq!(EN_US.convert(99.99, false), "Ninety Nine point Ninety Nine");
        assert_eq!(
            EN_US.convert(999.01, false),
            "Nine Hundred Ninety Nine point One"
        );
    }

    #[test]
    fn big_floating_point_numbers() {
        assert_eq!(
            EN_US.convert(69_420.69, false),
            "Sixty Nine Thousands Four Hundred Twenty point Sixty Nine"
        );
        assert_eq!(
            EN_US.convert(9_999_999_999_999.99, false),
            "Nine Trillions Nine Hundred Ninety Nine Billions Nine Hundred Ninety Nine Millions Nine Hundred Ninety Nine Thousands Nine Hundred Ninety Nine point Ninety Nine"
        );
        assert_eq!(
            EN_US.convert(1_457_923_567_901.12, false),
            "Trillion Four Hundred Fifty Seven Billions Nine Hundred Twenty Three Millions Five Hundred Sixty Seven Thousands Nine Hundred One point Twelve"
        );
        assert_eq!(
            EN_US.convert(2_000_000_000_000.20 as f64, false),
            "Two Trillions point Twenty"
        );
    }

    #[test]
    fn simple_money() {
        assert_eq!(EN_US.convert(0, true), "Zero Dollars");
        assert_eq!(EN_US.convert(1, true), "One Dollar");
        assert_eq!(EN_US.convert(10, true), "Ten Dollars");
        assert_eq!(EN_US.convert(11, true), "Eleven Dollars");
        assert_eq!(EN_US.convert(99, true), "Ninety Nine Dollars");
        assert_eq!(EN_US.convert(999, true), "Nine Hundred Ninety Nine Dollars");
    }

    #[test]
    fn floating_point_money() {
        assert_eq!(EN_US.convert(0.01, true), "Zero Dollars and One Cents");
        assert_eq!(EN_US.convert(1.99, true), "One Dollar and Ninety Nine Cent");
        assert_eq!(EN_US.convert(10.10, true), "Ten Dollars and Ten Cent");
        assert_eq!(EN_US.convert(11.11, true), "Eleven Dollars and Eleven Cent");
        assert_eq!(
            EN_US.convert(99.99, true),
            "Ninety Nine Dollars and Ninety Nine Cent"
        );
        assert_eq!(
            EN_US.convert(999.01, true),
            "Nine Hundred Ninety Nine Dollars and One Cents"
        );
    }

    #[test]
    fn big_money_numbers() {
        assert_eq!(
            EN_US.convert(69_420, true),
            "Sixty Nine Thousands Four Hundred Twenty Dollars"
        );
        assert_eq!(
            EN_US.convert(9_999_999_999_999_999 as i64, true),
            "Nine Quadrillions Nine Hundred Ninety Nine Trillions Nine Hundred Ninety Nine Billions Nine Hundred Ninety Nine Millions Nine Hundred Ninety Nine Thousands Nine Hundred Ninety Nine Dollars"
        );
        assert_eq!(
            EN_US.convert(1_457_923_567_901_234_567 as i64, true),
            "Quintillion Four Hundred Fifty Seven Quadrillions Nine Hundred Twenty Three Trillions Five Hundred Sixty Seven Billions Nine Hundred One Millions Two Hundred Thirty Four Thousands Five Hundred Sixty Seven Dollars"
        );
        assert_eq!(
            EN_US.convert(2_000_000_000_000_000_000 as i64, true),
            "Two Quintillions Dollars"
        );
    }

    #[test]
    fn big_floating_point_money_numbers() {
        assert_eq!(
            EN_US.convert(69_420.69, true),
            "Sixty Nine Thousands Four Hundred Twenty Dollars and Sixty Nine Cent"
        );
        assert_eq!(
            EN_US.convert(9_999_999_999_999.99, true),
            "Nine Trillions Nine Hundred Ninety Nine Billions Nine Hundred Ninety Nine Millions Nine Hundred Ninety Nine Thousands Nine Hundred Ninety Nine Dollars and Ninety Nine Cent"
        );
        assert_eq!(
            EN_US.convert(1_457_923_567_901.12, true),
            "Trillion Four Hundred Fifty Seven Billions Nine Hundred Twenty Three Millions Five Hundred Sixty Seven Thousands Nine Hundred One Dollars and Twelve Cent"
        );
        assert_eq!(
            EN_US.convert(2_000_000_000_000.20 as f64, true),
            "Two Trillions Dollars and Twenty Cent"
        );
    }
}
