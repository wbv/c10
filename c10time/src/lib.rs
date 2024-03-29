// SPDX-License-Identifier: MIT OR Apache-2.0

//! A library supporting C10 "decimalized" date and time.
//!
//! See also: <https://hackaday.io/project/11131-c10>

#![deny(warnings)]

#[cfg(test)]
mod tests;

use std::fmt;

pub mod epochs;

extern crate libc;
use libc::{clock_gettime, timespec, CLOCK_REALTIME};

pub const TICK: Duration = Duration::new(0, 0, 1);
pub const CENTIVAL: Duration = Duration::new(0, 1, 0);
pub const INTERVAL: Duration = Duration::new(1, 0, 0);
pub const DAY: Duration = Duration::new(100, 0, 0);
pub const DECADAY: Duration = Duration::new(10 * 100, 0, 0);

/// Representation for a unit of duration in C10 time.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Duration {
    ticks: u64,
}

impl Duration {
    /// Creates a [`Duration`] from constituent components: intervals, centivals, ticks.
    ///
    /// # Panics
    ///
    /// If any individual component or the sum of each cannot be represented as a `u64` of ticks,
    /// this constructor panics.
    /// _Note: this would correspond to a duration of ~50 billion years_
    pub const fn new(intervals: u64, centivals: u64, ticks: u64) -> Duration {
        // safely add intervals to ticks first
        let iticks = match intervals.checked_mul((100 * 100) as u64) {
            Some(iticks) => iticks,
            None => panic!("intervals overflow in Duration::new"),
        };
        let ticks = match ticks.checked_add(iticks) {
            Some(ticks) => ticks,
            None => panic!("intervals+ticks overflow in Duration::new"),
        };

        // then safely add centivals to ticks
        let cticks = match centivals.checked_mul(100_u64) {
            Some(cticks) => cticks,
            None => panic!("centivals overflow in Duration::new"),
        };
        let ticks = match ticks.checked_add(cticks) {
            Some(ticks) => ticks,
            None => panic!("centivals+ticks overflow in Duration::new"),
        };

        Duration { ticks }
    }

    /// Extracts the interval, centival, and tick components.
    pub fn time_components(&self) -> (u64, u64, u64) {
        let ticks = self.ticks % 100;
        let cents = (self.ticks / 100) % 100;
        let ints = (self.ticks / (100 * 100)) % 100;
        (ints, cents, ticks)
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, fmter: &mut fmt::Formatter) -> fmt::Result {
        let (ints, cents, ticks) = self.time_components();
        write!(fmter, "{ints:02}:{cents:02}:{ticks:02}")
    }
}

impl TryFrom<std::time::Duration> for Duration {
    type Error = std::num::TryFromIntError;

    fn try_from(duration: std::time::Duration) -> Result<Self, Self::Error> {
        let ticks: u128 = duration.as_micros() / 86_400;
        match ticks.try_into() {
            Ok(ticks) => Ok(Duration::new(0, 0, ticks)),
            Err(err) => Err(err),
        }
    }
}

/// A date and time of a local system in the decimalized C10 calendar and clock.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SystemTime {
    ticks: u64,
}

impl SystemTime {
    /// Gets the current system time as a SystemTime.
    ///
    /// # Panics
    ///
    /// This function panics if the underlying libc call fails or yields a result that is
    /// unrepresentable as a SystemTime.
    pub fn now() -> SystemTime {
        // get system time
        let (secs, nsecs) = {
            let ts: *mut timespec = std::mem::MaybeUninit::uninit().as_mut_ptr();

            // SAFETY: we verify the return value of the external function call was successful
            unsafe {
                match clock_gettime(CLOCK_REALTIME, ts) {
                    0 => ((*ts).tv_sec, (*ts).tv_nsec),
                    errno => panic!("clock_gettime failed with {errno}"),
                }
            }
        };

        // compute the number of ticks this way
        // 1 tick = 0.0864 seconds ==> 625 ticks = 54 seconds
        let sec_ticks = secs * 625 / 54;
        let nsec_ticks = nsecs * 625 / 54_000_000_000;

        SystemTime {
            ticks: (sec_ticks + nsec_ticks) as u64,
        }
    }

    /// Returns the interval, centival, and tick components of the timestamp's day.
    pub fn time_components(&self) -> (u64, u64, u64) {
        let ticks = self.ticks % 100;
        let cents = (self.ticks / 100) % 100;
        let ints = (self.ticks / (100 * 100)) % 100;
        (ints, cents, ticks)
    }

    /// Returns the year, decaday, and day components of the timestamp's date.
    pub fn date_components(&self) -> (u64, u64, u64) {
        let year: u64 = epochs::year_from_ticks(self.ticks).try_into().unwrap();
        let dayinyear = (self.ticks - epochs::year_to_ticks(year as usize)) / 1_000_000;
        let decaday = (dayinyear / 10) + 1;
        let day = (dayinyear % 10) + 1;
        (year, decaday, day)
    }
}

impl fmt::Display for SystemTime {
    fn fmt(&self, fmter: &mut fmt::Formatter) -> fmt::Result {
        let (ints, cents, ticks) = self.time_components();
        let (year, decaday, day) = self.date_components();
        write!(fmter, "{year:4} {decaday:2}.{day:02} ")?;
        write!(fmter, "{ints:02}:{cents:02}:{ticks:02}")
    }
}
