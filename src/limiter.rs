//! The core rate-limiting interface shared by every algorithm.

use std::time::Duration;

/// The outcome of a single rate-limit check.
///
/// A *denied* request is a normal, expected result — not an error — so it is a
/// variant here rather than an `Err`. Each variant carries the data a caller
/// actually needs to react.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    /// The request is permitted. `remaining` is how many further requests are
    /// still allowed before the limiter would start denying.
    Allowed { remaining: u64 },

    /// The request is rejected. `retry_after` is how long the caller should wait
    /// before a request would be permitted again (maps to the HTTP `Retry-After`
    /// header in the future web layer).
    Denied { retry_after: Duration },
}

impl Decision {
    /// Returns `true` if the request was allowed.
    ///
    /// ```
    /// use rate_limiter::limiter::Decision;
    /// use std::time::Duration;
    ///
    /// assert!(Decision::Allowed { remaining: 3 }.is_allowed());
    /// assert!(!Decision::Denied { retry_after: Duration::from_secs(1) }.is_allowed());
    /// ```
    pub fn is_allowed(&self) -> bool {
        matches!(self, Decision::Allowed { .. })
    }
}

/// A rate limiter decides whether a request, identified by an opaque `key`,
/// may proceed right now.
///
/// The `key` is whatever identity you rate-limit on: a user id, an IP address,
/// an API key, and so on. Implementors track per-key state internally.
pub trait RateLimiter {
    /// Records an attempt for `key` and returns whether it is [`Allowed`] or
    /// [`Denied`].
    ///
    /// Takes `&mut self` because checking a request updates the limiter's
    /// internal counters.
    ///
    /// [`Allowed`]: Decision::Allowed
    /// [`Denied`]: Decision::Denied
    fn check(&mut self, key: &str) -> Decision;
}
