# num2word

`num2word` is a Rust library for converting numbers to their word representation. It supports Polish and English (US) number formats, with optional currency representations. The library is designed for ease of use and provides accurate results up to trillions.

## Features

- Convert integers and floating-point numbers to words.
- Supports numbers up to trillions (with minor rounding issues for extremely large floating-point numbers).
- Predefined static languages for Polish (`pl`) and English (US) (`en_US`).
- Configurable through feature flags: `pl`, `en_US`, and `all`.

## Installation

To add `num2word` to your project, include it in your `Cargo.toml`
```toml
[dependencies]
num2word = { git = "https://github.com/Nyjako/num2word.git", branch = "master", features = ["pl", "en_US"] }
```

Enable the all feature if you want both Polish and English:
```toml
[dependencies]
num2word = { git = "https://github.com/Nyjako/num2word.git", branch = "master", features = ["all"] }
```

## Usage

### Example

To convert numbers to words, simply call the convert method on one of the predefined language structures:
```rust
use num2word;

fn main() {
    let value = 1234.56;
    
    // English conversion (with currency)
    let result = num2word::language::EN_US.convert(value, true);
    println!("In English: {}", result);
    
    // Polish conversion (without currency)
    let result = num2word::language::PL.convert(value, false);
    println!("In Polish: {}", result);
}
```

### Parameters

`value`:  The number to be converted. Supported types are `i128`, `i64`, `i32`, `f32`, and `f64`.
`is_money`: A boolean flag indicating whether the result should include the currency.

### Supported Languages

- **Polish (pl)**
- **English (en_US)**

### Limitations

- For floating-point numbers above a trillion, rounding errors may occur.
- Integer conversions are supported up to **Quintillion** (Can be later extended up to `i128` limit)

## License

This project, is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.