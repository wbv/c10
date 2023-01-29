// SPDX-License-Identifier: MIT OR Apache-2.0
#![deny(warnings)]

use super::*;


#[test]
fn zero_duration() {
    let zero = Duration::new(0, 0, 0);
    println!("It is: {zero}");
}

#[test]
fn one_day() {
    let day = DAY;
    println!("One day is: {day}");
}

#[test]
fn now() {
    let now = SystemTime::now();
    assert_ne!(now, SystemTime { ticks: 0 });
}

