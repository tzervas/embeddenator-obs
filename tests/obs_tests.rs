//! Integration tests for observability primitives

use embeddenator_obs::*;

#[test]
fn test_logging_basic() {
    logging::init();
    logging::warn("test warning");
}

#[test]
fn test_metrics_serialization() {
    let snap = metrics::MetricsSnapshot {
        poison_recoveries_total: 42,
        poison_path_inodes: 0,
        poison_inodes: 0,
        poison_inode_paths: 0,
        poison_directories: 0,
        poison_file_cache: 0,
        sub_cache_hits: 100,
        sub_cache_misses: 10,
        sub_cache_evictions: 0,
        index_cache_hits: 0,
        index_cache_misses: 0,
        index_cache_evictions: 0,
        retrieval_query_calls: 50,
        retrieval_query_ns_total: 1000000,
        retrieval_query_ns_max: 50000,
        rerank_calls: 0,
        rerank_ns_total: 0,
        rerank_ns_max: 0,
        hier_query_calls: 0,
        hier_query_ns_total: 0,
        hier_query_ns_max: 0,
    };
    
    let json = serde_json::to_string(&snap).unwrap();
    let deserialized: metrics::MetricsSnapshot = serde_json::from_str(&json).unwrap();
    
    assert_eq!(deserialized.poison_recoveries_total, 42);
    assert_eq!(deserialized.sub_cache_hits, 100);
}
