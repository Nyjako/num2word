# Contributing to num2word

Thank you for considering contributing to `num2word`! Whether you're adding a new language, fixing bugs, or implementing new features, your contribution is greatly appreciated. Please follow the guidelines below to help ensure that your contribution is properly integrated.

## Adding a New Language

To add support for a new language, follow these steps:

1. **Fork the Repository**
    Fork the `num2word` repository to your own GitHub account.
2. **Use the Template**
    In your fork, navigate to `src/language/template.rs`. Use this file as a template for your new language implementation.
3. **Write Tests**
    Write unit tests for your new language. Refer to the existing tests for Polish (`pl`) and English (`en_US`) for guidance.
4. **Add a Feature**
    Add a new feature for your language in `Cargo.toml`. Make sure to also append your new feature to the `"all"` feature set so it is included when all languages are enabled:
    ```toml
    [features]
    new_language = []
    all = ["pl", "en_US", "new_language"]
    ```
5. **Update `mod.rs`**
    Edit `src/language/mod.rs` to include new language. It should look something like this:
    ```rust
    #[cfg(feature = "en_US")]
    pub mod en_us;
    #[cfg(feature = "en_US")]
    pub use crate::language::en_us::EN_US as EN_US;
    
    #[cfg(feature = "new_language")]
    pub mod new_language;
    #[cfg(feature = "new_language")]
    pub use crate::language::new_language::NEW_LANGUAGE as NEW_LANGUAGE;
    ```
6. **Run Tests**
    Ensure that your implementation works and nothink broke by running the tests:
    ```bash
    cargo test
    ```
7. **Open a Pull Request**
    Once everything is working and tested, push your changes to your fork and [open a pull request](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/creating-a-pull-request) against the main repository.

## Fixing Bugs and Adding Features

For bug fixes and feature additions:

1. Fork the repository.
2. Make your changes and ensure that they don't break any existing functionality.
3. Write tests for any new features or bug fixes.
4. Run `cargo test` to verify all tests pass.
5. Open a pull request with a clear description of your changes.

## Code Style

- Follow the existing coding style in the project.
- Ensure that your code is properly formatted by running `cargo fmt`.
- Run `cargo clippy` to catch common mistakes and improve code quality.

## Communication

If you're working on a significant change, feel free to [open an issue](https://github.com/Nyjako/num2word/issues) to discuss your ideas before making large modifications.

Thank you for contributing!