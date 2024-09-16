pub mod language;

#[derive(Debug, Clone)]
pub struct Language {
    currency: Currency,
    texts: Texts,
    word_mapping: Vec<WordsMapping>,
}

#[derive(Debug, Clone, Copy)]
pub struct Currency {
    // name: &'static str,
    plural: &'static str,
    singular: &'static str,
    // symbol: &'static str,
    fractional_unit: FractionalUnit,
}

#[derive(Debug, Clone, Copy)]
pub struct FractionalUnit {
    // name: &'static str,
    plural: &'static str,
    singular: &'static str,
    // symbol: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct Texts {
    and: &'static str,
    minus: &'static str,
    // only: &'static str,
    point: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct WordsMapping {
    number: i128,
    value: &'static str,
    plural: Option<&'static str>,
}

fn find_value_in_word_map(value: i128, language: &Language, use_plural: bool) -> String {
    if let Some(result) = language.word_mapping.iter().find(|&r| r.number == value) {
        if use_plural {
            if let Some(plural) = result.plural {
                return plural.to_string();
            }
        }

        return result.value.to_string();
    }

    if value <= 20 {
        panic!("No mapping found for value {}", value);
    }

    let mut result = String::new();
    let mut temp_value = value;

    for val_map in &language.word_mapping {
        if temp_value >= val_map.number && val_map.number != 0 {
            let temp = (temp_value / val_map.number) * val_map.number;
            let count = temp / val_map.number;

            if count > 1 {
                result.push_str(&find_value_in_word_map(count, language, false));
                result.push(' ');
            }
            result.push_str(&find_value_in_word_map(val_map.number, language, count > 1));
            result.push(' ');

            temp_value -= temp;
        }
    }

    return result.trim().to_string();
}

pub trait Convertible {
    fn convert_to_string(&self, language: &Language, is_money: bool) -> String;
}

impl Convertible for i128 {
    fn convert_to_string(&self, language: &Language, is_money: bool) -> String {
        if *self == 0 {
            return if is_money {
                format!(
                    "{} {}",
                    find_value_in_word_map(*self, language, false),
                    language.currency.plural
                )
            } else {
                find_value_in_word_map(*self, language, false)
            };
        }

        let mut result = String::new();
        let mut num = *self;

        if num < 0 {
            result.push_str(language.texts.minus);
            result.push(' ');
            num = num.abs();
        }

        result.push_str(&find_value_in_word_map(num, language, false));

        if is_money {
            result.push(' ');
            result.push_str(if *self == 1 {
                language.currency.singular
            } else {
                language.currency.plural
            });
        }

        result.trim().to_string()
    }
}

impl Convertible for i64 {
    fn convert_to_string(&self, language: &Language, is_money: bool) -> String {
        (*self as i128).convert_to_string(language, is_money)
    }
}

impl Convertible for i32 {
    fn convert_to_string(&self, language: &Language, is_money: bool) -> String {
        (*self as i128).convert_to_string(language, is_money)
    }
}

impl Convertible for f64 {
    fn convert_to_string(&self, language: &Language, is_money: bool) -> String {
        let mut result = String::new();

        let scaled_value = (*self * 100.0).round() as i128;
        let integer_part = scaled_value / 100;
        let fractional_part = scaled_value % 100;

        result.push_str(&integer_part.convert_to_string(language, false));
        if is_money {
            result.push(' ');
            result.push_str(if integer_part == 1 {
                language.currency.singular
            } else {
                language.currency.plural
            });
        }

        if fractional_part > 0 {
            result.push_str(&format!(
                " {} {}",
                if is_money {
                    language.texts.and
                } else {
                    language.texts.point
                },
                fractional_part.convert_to_string(language, false)
            ));
            if is_money {
                result.push(' ');
                result.push_str(if fractional_part == 1 {
                    language.currency.fractional_unit.singular
                } else {
                    language.currency.fractional_unit.plural
                });
            }
        }

        result
    }
}

impl Convertible for f32 {
    fn convert_to_string(&self, language: &Language, is_money: bool) -> String {
        (*self as f64).convert_to_string(language, is_money)
    }
}

impl Language {
    pub fn convert<T: Convertible>(&self, value: T, is_money: bool) -> String {
        value.convert_to_string(self, is_money)
    }
}
