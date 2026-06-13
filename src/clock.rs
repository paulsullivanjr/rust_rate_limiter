//! Time abstraction.
//!
//! Every rate-limiting algorithm needs to know "what time is it now?". Instead of
//! calling [`Instant::now`] directly (which is impossible to test deterministically),
//! algorithms depend on the [`Clock`] trait. Production code uses [`SystemClock`];
//! tests use a mock clock whose time can be advanced by hand.

use std::time::Instant;

/// A source of the current instant in time.
///
/// Implementors answer one question: "what time is it now?". This indirection is
/// what lets us swap real time for controllable time in tests.
pub trait Clock {
    /// Returns the current instant according to this clock.
    fn now(&self) -> Instant;
}

/// A [`Clock`] backed by the operating system's monotonic clock.
///
/// This is the clock you use in production. It simply delegates to
/// [`Instant::now`].
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> Instant {
        Instant::now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_clock_never_goes_backwards() {
        let clock = SystemClock;
        let first = clock.now();
        let second = clock.now();
        // `Instant` is monotonic: a later reading is never earlier than an earlier one.
        assert!(second >= first);
    }
}
