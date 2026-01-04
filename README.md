# embeddenator-obs

Observability: metrics, logging, and high-resolution timing for Embeddenator.

## Features

- **Metrics**: Thread-safe atomic metrics collection (cache stats, query timings, poison recovery tracking)
- **Logging**: Optional structured logging via tracing
- **High-Resolution Timing**: Microsecond-precision timers for performance measurement

## Usage

### Metrics

```rust
use embeddenator_obs::metrics;

let m = metrics::Metrics::new();

// Record cache operations
m.inc_sub_cache_hit();
m.inc_sub_cache_miss();

// Record query timing
use std::time::Duration;
m.record_retrieval_query(Duration::from_millis(5));

// Get snapshot
let snap = m.snapshot();
println!("Cache hits: {}", snap.sub_cache_hits);
println!("Queries: {}", snap.retrieval_query_calls);
```

### Logging (Optional Feature)

```toml
[dependencies]
embeddenator-obs = { version = "0.2", features = ["logging"] }
```

```rust
use embeddenator_obs::logging;

logging::init(); // Initialize from EMBEDDENATOR_LOG or RUST_LOG
logging::warn("Something important happened");
```

Without the feature, logging is no-op.

### High-Resolution Timing

```rust
use embeddenator_obs::hires_timing;

let timer = hires_timing::Timer::start();
// ... do work ...
let elapsed_us = timer.elapsed_us();
println!("Operation took {} μs", elapsed_us);
```

## Metrics Available

- **Poison Recovery**: RwLock poison recovery tracking
- **Cache Stats**: Sub-engram and index cache hits/misses/evictions
- **Query Timings**: Retrieval, rerank, hierarchical query durations

## Security Audit

**Status:** ⚠️ **2 safe unsafe blocks**

- `src/obs/hires_timing.rs`: CPU intrinsics for TSC (`_rdtsc()`)
  - x86/x86_64 time stamp counter access
  - Safe when used for timing (no memory unsafety)

## License

MIT

## Part of Embeddenator

This is a component of the [Embeddenator](https://github.com/tzervas/embeddenator) holographic computing substrate.
