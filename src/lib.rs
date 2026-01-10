//! # embeddenator-obs
//!
//! Observability: metrics, logging, and tracing for Embeddenator.
//!
//! Extracted from embeddenator core as part of Phase 2A component decomposition.

pub mod obs;
pub use obs::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_loads() {
        // Verify core types are accessible
        let metrics = Metrics::new();
        let snapshot = metrics.snapshot();
        assert_eq!(snapshot.poison_recoveries_total, 0);
    }
}
