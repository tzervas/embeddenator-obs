# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.21.0] - 2026-01-25

### Added
- Prometheus metrics export (text format) via `prometheus-export` feature
- OpenTelemetry distributed tracing (W3C Trace Context) via `opentelemetry-tracing` feature
- Advanced statistics module (percentiles, std dev, histograms) via `advanced-stats` feature
- Real-time metric streaming with callbacks

### Changed
- All new features are optional via feature flags for minimal binary size

## [0.20.0-alpha.1] - 2026-01-16

### Added
- Initial alpha release
- Core metrics collection
- Basic logging integration
- Tracing support foundation
