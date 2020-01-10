// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

//! # fake_clock
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
    exceeding_bitshifts,
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
    plugin_as_library,
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

use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Sub};
use std::time::Duration;

/// Struct representing a fake instant
#[derive(Clone, Copy)]
pub struct FakeClock {
    time_created: u64,
}

thread_local! {
    static LOCAL_TIME: RefCell<u64> = RefCell::new(0);
}

impl FakeClock {
    /// Sets the thread-local fake time to the given value
    pub fn set_time(time: u64) {
        LOCAL_TIME.with(|t| {
            *t.borrow_mut() = time;
        });
    }

    /// Advances the thread-local fake time by the given amount of milliseconds
    pub fn advance_time(millis: u64) {
        LOCAL_TIME.with(|t| {
            *t.borrow_mut() += millis;
        });
    }

    /// Returns the current thread-local fake time
    pub fn time() -> u64 {
        LOCAL_TIME.with(|t| *t.borrow())
    }

    /// Returns a `FakeClock` instance representing the current instant.
    pub fn now() -> Self {
        let time = Self::time();
        FakeClock { time_created: time }
    }

    /// Returns the duration that passed between `self` and `earlier`.
    pub fn duration_since(self, earlier: Self) -> Duration {
        Duration::from_millis(self.time_created - earlier.time_created)
    }

    /// Returns how much fake time has elapsed since the creation of `self`.
    pub fn elapsed(self) -> Duration {
        Duration::from_millis(Self::time() - self.time_created)
    }
}

impl PartialEq for FakeClock {
    fn eq(&self, other: &FakeClock) -> bool {
        self.time_created == other.time_created
    }
}

impl Hash for FakeClock {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.time_created.hash(state);
    }
}

impl Eq for FakeClock {}

impl PartialOrd for FakeClock {
    fn partial_cmp(&self, other: &FakeClock) -> Option<Ordering> {
        self.time_created.partial_cmp(&other.time_created)
    }
}

impl Ord for FakeClock {
    fn cmp(&self, other: &FakeClock) -> Ordering {
        self.time_created.cmp(&other.time_created)
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
    type Output = FakeClock;
    fn add(mut self, other: Duration) -> FakeClock {
        self.time_created += other.as_secs() * 1000 + u64::from(other.subsec_nanos()) / 1_000_000;
        self
    }
}

impl Sub<Duration> for FakeClock {
    type Output = FakeClock;
    fn sub(mut self, other: Duration) -> FakeClock {
        self.time_created -= other.as_secs() * 1000 + u64::from(other.subsec_nanos()) / 1_000_000;
        self
    }
}

impl Sub<FakeClock> for FakeClock {
    type Output = Duration;
    fn sub(self, other: FakeClock) -> Duration {
        Duration::from_millis(self.time_created - other.time_created)
    }
}
