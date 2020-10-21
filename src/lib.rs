// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

//! # sn_fake_clock
//!
//! A crate providing a virtual clock mimicking `std::time::Instant`'s interface, enabling full
//! control over the flow of time during testing.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/maidsafe/QA/master/Images/maidsafe_logo.png",
    html_favicon_url = "https://maidsafe.net/img/favicon.ico",
    test(attr(forbid(warnings)))
)]
// For explanation of lint checks, run `rustc -W help` or see
// https://github.com/maidsafe/QA/blob/master/Documentation/Rust%20Lint%20Checks.md
#![forbid(
    bad_style,
    arithmetic_overflow,
    mutable_transmutes,
    no_mangle_const_items,
    unknown_crate_types,
    warnings
)]
#![deny(
    deprecated,
    improper_ctypes,
    missing_docs,
    non_shorthand_field_patterns,
    overflowing_literals,
    stable_features,
    unconditional_recursion,
    unknown_lints,
    unsafe_code,
    unused,
    unused_allocation,
    unused_attributes,
    unused_comparisons,
    unused_features,
    unused_parens,
    while_true
)]
#![warn(
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]
#![allow(
    box_pointers,
    missing_copy_implementations,
    missing_debug_implementations,
    variant_size_differences
)]

use std::cell::Cell;
use std::convert::TryInto;
use std::fmt;
use std::ops::{Add, Sub};
use std::time::Duration;

/// Struct representing a fake instant
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FakeClock {
    time_created: u64,
}

thread_local! {
    static LOCAL_TIME: Cell<u64> = Cell::new(0);
}

impl FakeClock {
    /// Sets the thread-local fake time to the given value
    pub fn set_time(time: u64) {
        LOCAL_TIME.with(|t| {
            t.set(time);
        });
    }

    /// Advances the thread-local fake time by the given amount of milliseconds
    pub fn advance_time(millis: u64) {
        LOCAL_TIME.with(|t| {
            t.set(t.get() + millis);
        });
    }

    /// Returns the current thread-local fake time
    pub fn time() -> u64 {
        LOCAL_TIME.with(|t| t.get())
    }

    /// Returns a `FakeClock` instance representing the current instant.
    pub fn now() -> Self {
        let time = Self::time();
        Self { time_created: time }
    }

    /// Returns the duration that passed between `self` and `earlier`.
    pub fn duration_since(self, earlier: Self) -> Duration {
        Duration::from_millis(self.time_created - earlier.time_created)
    }

    /// Returns how much fake time has elapsed since the creation of `self`.
    pub fn elapsed(self) -> Duration {
        Duration::from_millis(Self::time() - self.time_created)
    }

    /// Returns `Some(t)` where `t` is the time `self + duration` if `t` can be
    /// represented as `FakeClock`, `None` otherwise.
    pub fn checked_add(&self, duration: Duration) -> Option<FakeClock> {
        duration
            .as_millis()
            .checked_add(self.time_created as u128)
            .and_then(|time| time.try_into().ok())
            .map(|time| Self { time_created: time })
    }

    /// Returns `Some(t)` where `t` is the time `self - duration` if `t` can be
    /// represented as `FakeClock`, `None` otherwise.
    pub fn checked_sub(&self, duration: Duration) -> Option<FakeClock> {
        duration
            .as_millis()
            .try_into()
            .ok()
            .and_then(|dur| self.time_created.checked_sub(dur))
            .map(|time| Self { time_created: time })
    }
}

impl fmt::Debug for FakeClock {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "FakeClock {{ time_created: {} }}",
            self.time_created
        )
    }
}

impl Add<Duration> for FakeClock {
    type Output = Self;
    fn add(mut self, other: Duration) -> FakeClock {
        self.time_created += other.as_millis() as u64;
        self
    }
}

impl Sub<Duration> for FakeClock {
    type Output = Self;
    fn sub(mut self, other: Duration) -> FakeClock {
        self.time_created -= other.as_millis() as u64;
        self
    }
}

impl Sub<FakeClock> for FakeClock {
    type Output = Duration;
    fn sub(self, other: FakeClock) -> Duration {
        Duration::from_millis(self.time_created - other.time_created)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checked_add_some() {
        FakeClock::set_time(0);

        let inst = FakeClock::now();
        let dur = Duration::from_millis(std::u64::MAX);
        FakeClock::set_time(std::u64::MAX);

        assert_eq!(Some(FakeClock::now()), inst.checked_add(dur));
    }

    #[test]
    fn test_checked_add_none() {
        FakeClock::set_time(1);

        let inst = FakeClock::now();
        let dur = Duration::from_millis(std::u64::MAX);

        assert_eq!(None, inst.checked_add(dur));
    }

    #[test]
    fn test_checked_sub_some() {
        FakeClock::set_time(std::u64::MAX);

        let inst = FakeClock::now();
        let dur = Duration::from_millis(std::u64::MAX);
        FakeClock::set_time(0);

        assert_eq!(Some(FakeClock::now()), inst.checked_sub(dur));
    }

    #[test]
    fn test_checked_sub_none() {
        FakeClock::set_time(std::u64::MAX - 1);

        let inst = FakeClock::now();
        let dur = Duration::from_millis(std::u64::MAX);

        assert_eq!(None, inst.checked_sub(dur));
    }
}
