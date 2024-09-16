// mod template;

#[cfg(feature = "pl")]
pub mod pl;
#[cfg(feature = "pl")]
pub use crate::language::pl::PL;

#[cfg(feature = "en_US")]
pub mod en_us;
#[cfg(feature = "en_US")]
pub use crate::language::en_us::EN_US;
