//! Searching algorithms.

#[cfg(test)]
#[macro_use]
mod test_cases;

mod linear_search;
pub use self::linear_search::linear_search;

mod binary_search;
pub use self::binary_search::binary_search;

mod interpolation_search;
pub use self::interpolation_search::interpolation_search;

mod exponential_search;
pub use self::exponential_search::exponential_search;
