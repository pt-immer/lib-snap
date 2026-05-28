//! axum/tower inbound-verify layer for SNAP BI service signatures.
//!
//! Provides a `tower::Layer` that wraps inner services and rejects requests
//! whose SNAP BI signature fails to validate against a caller-supplied
//! client-secret lookup.

#![forbid(unsafe_code)]
