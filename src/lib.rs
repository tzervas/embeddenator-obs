//! # embeddenator-obs
//!
//! Observability: metrics, logging, and tracing for Embeddenator.
//!
//! Extracted from embeddenator core as part of Phase 2A component decomposition.

pub mod obs;
pub use obs::*;

#[cfg(test)]
mod tests {
    #[test]
    fn component_loads() {
        assert!(true);
    }
}
