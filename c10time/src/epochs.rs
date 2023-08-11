//! Module for handling conversions from Unix time and its associated epoch of New Years 1970.

#[allow(unused)]
macro_rules! is_leap {
    ($year:expr) => {
        ($year % 4 == 0 && ($year % 100 != 0 || $year % 400 == 0)) as bool
    };
}

/// Days since the Unix epoch for a given year on January 1.
pub const fn year_to_days(year: usize) -> usize {
    let mut days = 0;
    let mut year = year;
    while year > 1970 {
        days += 365;
        if is_leap!(year) {
            days += 1;
        }
        year -= 1;
    }
    days
}

/// Seconds after the Unix epoch for a given year.
pub const fn year_to_seconds(year: usize) -> usize {
    year_to_days(year) * (24 * 60 * 60)
}

/// Ticks after the Unix epoch for a given year.
pub const fn year_to_ticks(year: usize) -> u64 {
    year_to_days(year) as u64 * (100 * 100 * 100)
}

/// Returns the year to which a given Unix epoch time (seconds) belongs.
pub const fn year_from_seconds(secs: usize) -> usize {
    let mut guess = 1970;
    while year_to_seconds(guess + 1) < secs {
        guess += 1;
    }
    guess
}

/// Returns the year to which a given c10 tick belongs.
pub const fn year_from_ticks(ticks: u64) -> usize {
    let mut guess = 1970;
    while year_to_ticks(guess + 1) < ticks {
        guess += 1;
    }
    guess
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epoch_lookup() {
        assert_eq!(year_to_seconds(2023), 1672531200);
    }

    #[test]
    fn epoch_lookup_ticks() {
        assert_eq!(year_to_ticks(1971), 365_000_000);
    }

    #[test]
    fn find_year_from_secs() {
        let year = year_from_seconds(1672531200 + 250000);
        assert_eq!(2023, year);
    }
}
