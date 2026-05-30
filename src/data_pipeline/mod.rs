//! Data pipeline for Stellar ledger stream processing and ETL (#788).
//!
//! Architecture:
//! ```
//! Kafka Source → ETL Transform → Schema Validation → Multi-Sink Fan-out
//!                                                   ├─ PostgreSQL
//!                                                   ├─ Elasticsearch
//!                                                   └─ S3
//!                     ↓ on error
//!                Dead Letter Queue (Kafka DLQ topic)
//! ```
//!
//! Features:
//! - Kafka-based event streaming with consumer groups
//! - Real-time ETL transformations with schema validation
//! - Data quality checks
//! - Multiple sink connectors (PostgreSQL, Elasticsearch, S3)
//! - Data lineage tracking and audit trail
//! - Pipeline monitoring (throughput, latency, error metrics)
//! - Dead letter queue for failed records

pub mod config;
pub mod etl;
pub mod lineage;
pub mod metrics;
pub mod pipeline;
pub mod sinks;

pub use config::PipelineConfig;
pub use pipeline::{DataPipeline, PipelineHandle};
pub use etl::{EtlRecord, TransformError};
pub use lineage::LineageTracker;
pub use metrics::PipelineMetrics;
pub use sinks::SinkError;
