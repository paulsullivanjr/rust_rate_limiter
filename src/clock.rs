//! Time abstraction.
//!
//! Every rate-limiting algorithm needs to know "what time is it now?". Instead of
//! calling [`Instant::now`] directly (which is impossible to test deterministically),
//! algorithms depend on the [`Clock`] trait. Production code uses [`SystemClock`];
//! tests use a mock clock whose time can be advanced by hand.

use std::cell::Cell;
use std::rc::Rc;
use std::time::{Duration, Instant};

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

/// A controllable [`Clock`] for tests.
///
/// Time only moves when you call [`advance`](MockClock::advance) — never on its
/// own — which makes time-dependent behavior deterministic and instant to test
/// (no `sleep`). Every [`clone`](Clone::clone) shares the *same* underlying time,
/// so you can hand a clone to a rate limiter and still advance time from the
/// original handle.
///
/// ```
/// use rate_limiter::clock::{Clock, MockClock};
/// use std::time::Duration;
///
/// let clock = MockClock::new();
/// let start = clock.now();
/// clock.advance(Duration::from_secs(3));
/// assert_eq!(clock.now(), start + Duration::from_secs(3));
/// ```
#[derive(Clone)]
pub struct MockClock {
    // `Rc`  -> shared ownership: clones point at the same value.
    // `Cell` -> interior mutability: we can update the time through `&self`.
    now: Rc<Cell<Instant>>,
}

impl MockClock {
    /// Creates a mock clock starting at the current real instant.
    pub fn new() -> Self {
        Self {
            now: Rc::new(Cell::new(Instant::now())),
        }
    }

    /// Moves the clock forward by `delta`. Takes `&self` (not `&mut self`) thanks
    /// to the interior mutability of `Cell`.
    pub fn advance(&self, delta: Duration) {
        self.now.set(self.now.get() + delta);
    }
}

impl Default for MockClock {
    fn default() -> Self {
        Self::new()
    }
}

impl Clock for MockClock {
    fn now(&self) -> Instant {
        self.now.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn system_clock_never_goes_backwards() {
        let clock = SystemClock;
        let first = clock.now();
        let second = clock.now();
        // `Instant` is monotonic: a later reading is never earlier than an earlier one.
        assert!(second >= first);
    }

    #[test]
    fn mock_clock_advances_only_when_told() {
        let clock = MockClock::new();
        let start = clock.now();

        // Time does not move on its own.
        assert_eq!(clock.now(), start);

        // ...but it moves exactly as far as we advance it.
        clock.advance(Duration::from_secs(5));
        assert_eq!(clock.now(), start + Duration::from_secs(5));
    }

    #[test]
    fn mock_clock_clones_share_the_same_time() {
        let clock = MockClock::new();
        let handle = clock.clone(); // a second owner of the SAME underlying time

        clock.advance(Duration::from_secs(1));

        // Advancing through one handle is visible through the other:
        // `Rc` means they share one value, not two copies.
        assert_eq!(handle.now(), clock.now());
    }
}
