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
