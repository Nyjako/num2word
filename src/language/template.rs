use lazy_static::lazy_static;
use crate::{ Language, Currency, FractionalUnit, Texts, WordsMapping };

lazy_static! {
    pub static ref TEMPLATE: Language = Language { // Commented fields are for things what would be added later
        currency: Currency { 
            // name: "", 
            plural: "", 
            singular: "", 
            // symbol: "", 
            fractional_unit: FractionalUnit { 
                // name: "", 
                plural: "", 
                singular: "", 
                // symbol: "",
            },
        },
        texts: Texts {
            and: "",
            minus: "",
            // only: "",
            point: "",
        },
        word_mapping: vec![
            WordsMapping {number: 0, value: "", plural: None},
        ],
    };
}