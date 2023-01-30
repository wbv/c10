//! Module for handling conversions from Unix time and its associated epoch of New Years 1970.

/// Seconds elapsed since Unix Epoch, indexed by year (pulled from GNU `date` utility) starting
/// with 1970.
///
/// Inspection after-the-fact suggested that the every-fourth-year leap-year is followed, so this
/// could be programmatically generated, but I'm trying not to pretend I'm smarted than the
/// standard library developers.
pub const EPOCH_SECONDS: [usize; 70] = [
    0, 31536000, 63072000, 94694400, 126230400, 157766400, 189302400, 220924800, 252460800,
    283996800, 315532800, 347155200, 378691200, 410227200, 441763200, 473385600, 504921600,
    536457600, 567993600, 599616000, 631152000, 662688000, 694224000, 725846400, 757382400,
    788918400, 820454400, 852076800, 883612800, 915148800, 946684800, 978307200, 1009843200,
    1041379200, 1072915200, 1104537600, 1136073600, 1167609600, 1199145600, 1230768000, 1262304000,
    1293840000, 1325376000, 1356998400, 1388534400, 1420070400, 1451606400, 1483228800, 1514764800,
    1546300800, 1577836800, 1609459200, 1640995200, 1672531200, 1704067200, 1735689600, 1767225600,
    1798761600, 1830297600, 1861920000, 1893456000, 1924992000, 1956528000, 1988150400, 2019686400,
    2051222400, 2082758400, 2114380800, 2145916800, 2177452800,
];

/// Seconds after the Unix epoch for a given year.
pub const fn year_to_seconds(year: usize) -> usize {
    let index = year - 1970;
    assert!(index < EPOCH_SECONDS.len());
    EPOCH_SECONDS[index]
}

/// C10 ticks after the Unix epoch for a given year.
pub const fn year_to_ticks(year: usize) -> u64 {
    year_to_seconds(year) as u64 * 1_000_000 / 86_400
}

/// Returns the year to which a given Unix epoch time (seconds) belongs.
pub fn year_from_secs(secs: usize) -> usize {
    // Premature optimization:
    // Guess an index to search the list faster, costing only a single logical shift.
    //
    // Explanation: there are slightly more than 2^25 seconds in a year, so this will compute a
    // year/index in the array which is below where we're aiming, but close enough that a linear
    // search will still be relatively fast.
    let mut index = secs >> 25;
    while (EPOCH_SECONDS[index + 1] as usize) < secs {
        index += 1;
        if index >= EPOCH_SECONDS.len() {
            panic!("W: index above bounds!");
        }
    }
    index + 1970
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
        assert_ne!(year_to_ticks(2023), 0);
    }

    #[test]
    fn find_year_from_secs() {
        let year = year_from_secs(1672531200 + 250000);
        assert_eq!(2023, year);
    }
}
