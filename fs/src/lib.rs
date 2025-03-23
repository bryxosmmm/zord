mod macros;
mod structs;
#[cfg(test)]
mod test;
mod traits;
mod utils;

pub use crate::structs::Files;
pub use crate::structs::Metadata;
pub use crate::traits::Collector;
