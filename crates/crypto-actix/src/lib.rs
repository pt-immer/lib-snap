//! actix-web inbound-verify middleware for SNAP BI service signatures.
//!
//! Wraps [`kamu_snap_crypto::snap_bi::verify_service`] in an `actix-web`
//! `Transform` so handlers behind the middleware receive only requests whose
//! `X-SIGNATURE` header validates against a caller-supplied client-secret
//! lookup function.

#![forbid(unsafe_code)]
