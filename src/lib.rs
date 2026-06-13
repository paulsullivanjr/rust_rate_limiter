//! A production-grade rate limiter library.
//!
//! This crate provides multiple rate-limiting algorithms behind a common
//! [`RateLimiter`](limiter::RateLimiter) trait. All algorithms read the current
//! time through the [`Clock`](clock::Clock) trait, which makes time-dependent
//! behavior fully testable.

pub mod clock;
