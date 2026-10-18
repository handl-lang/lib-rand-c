//! `rand-c` — the libsodium `getrandom` backend for random values.
//!
//! `entropy.hnl` is the low-level FFI surface (`next_u32`); `index.hnl`
//! builds the `rand` arrows and distributions on top of it.

/// Low-level libsodium entropy (`next_u32`).
pub const ENTROPY_HNL: &str = include_str!("entropy.hnl");

/// The `rand-c` package entry.
pub const RAND_C_HNL: &str = include_str!("index.hnl");
