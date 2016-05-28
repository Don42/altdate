
/// Enum containing all discordian Days, including StTibsDay
#[derive(Debug,PartialEq)]
enum Day {
    Sweetmorn,
    Boomtime,
    Pungenday,
    PricklePrickle,
    SettingOrange,
    StTibsDay,
}

/// Enum containing all discordian Seasons, including StTibsDay
#[derive(Debug,PartialEq)]
enum Season {
    Chaos,
    Discord,
    Confusion,
    Bureaucracy,
    TheAftermath,
    StTibsDay,
}


/// Representation for a Discordian Date
#[derive(Debug,PartialEq)]
pub struct DiscordianDate {
    /// Season of the discordian year
    season: Season,
    /// Day of the discordian Season, zero-based
    day: u8,
    /// Day of the discordian year, zero-based
    year_day: u16,
    /// Discordian year, which includes a year zero
    year: i32,
    /// Day of the discordian week
    week_day: Day,
    /// Week of the discordian year, or None for StTibsDay
    week: Option<u8>,
}


/// Converts a year and day to a Discordian Date
///
/// # Arguments
/// * `nday` - Days after January 1st, starting at zero
/// * `nyear` - Astronomicaly numbered year. This means there is a year zero
///
pub fn convert(nday: u16, nyear: i32) -> Option<DiscordianDate> {
    let year = nyear + 1166;
    let year_day = nday;

    if !is_leap_year(nyear) {
        let season = match nday {
            0 ... 72 => Season::Chaos,
            73 ... 145 => Season::Discord,
            146 ... 218 => Season::Confusion,
            219 ... 291 => Season::Bureaucracy,
            292 ... 364 => Season::TheAftermath,
            _ => panic!("Day out of range: {}", nday)
        };

        let week_day = week_day(nday);
        let day = (nday % 73) as u8;
        let week = Some((nday / 5) as u8);
        return Some(DiscordianDate {season: season, day: day,
                             year_day: year_day, year: year,
                             week: week, week_day: week_day})
    } else {
        let season = match nday {
            59 => Season::StTibsDay,
            0 ... 73 => Season::Chaos,
            74 ... 146 => Season::Discord,
            147 ... 219 => Season::Confusion,
            220 ... 292 => Season::Bureaucracy,
            293 ... 365 => Season::TheAftermath,
            _ => panic!("Day out of range: {}", nday)
        };

        let week_day = match nday {
                0 ... 58 => week_day(nday),
                59 => Day::StTibsDay,
                60 ... 365 => week_day(nday - 1),
                _ => panic!("Day out of range: {}", nday)
        };

        let day = match nday {
                0 ... 58 => nday,
                59 => 0,
                60 ... 365 => (nday - 1) % 73,
                _ => panic!("Day out of range: {}", nday)
        } as u8;

        let week = match nday {
                0 ... 58 => Some((nday / 5) as u8),
                59 => None,
                60 ... 365 => Some(((nday - 1) / 5) as u8),
                _ => panic!("Day out of range: {}", nday)
        };

        return Some(DiscordianDate {season: season, day: day,
                             year_day: year_day, year: year,
                             week: week, week_day: week_day})

    }

}


/// Return the weekday for a given day in the discordian year
///
/// This function will not correct for StTibsDay. All dates after StTibsDay
/// need to be reduced by one.
///
/// # Arguments
/// * `nday` - Days after January 1st, starting at zero
///
fn week_day(nday: u16) -> Day{
    match nday % 5 {
        0 => Day::Sweetmorn,
        1 => Day::Boomtime,
        2 => Day::Pungenday,
        3 => Day::PricklePrickle,
        4 => Day::SettingOrange,
        _ => panic!("Weekday out of range: {}", nday % 5)
    }
}


/// Determines if the supplied year is a leap year
///
/// There is a year zero before year one. But the result of the
/// leap year calculation is undefined before the switch to the
/// gregorian calendar (1582 CE)
///
/// # Arguments
/// * `year` - Astronomicaly numbered year. This means there is a year zero
///
fn is_leap_year(year: i32) -> bool {
    let has_factor = |n| year % n == 0;
    return has_factor(4) && !has_factor(100) || has_factor(400)
}

#[cfg(test)]
mod test {

    #[test]
    fn test_convert() {
        assert_eq!(super::DiscordianDate {season: super::Season::Chaos,
                                          day: 0, year_day: 0, year: 3182,
                                          week: Some(0), week_day: super::Day::Sweetmorn},
                                          super::convert(0, 2016).unwrap());
        assert_eq!(super::DiscordianDate {season: super::Season::Chaos,
                                          day: 0, year_day: 0, year: 1166,
                                          week: Some(0), week_day: super::Day::Sweetmorn},
                                          super::convert(0, 0).unwrap());
        assert_eq!(super::DiscordianDate {season: super::Season::Chaos,
                                          day: 0, year_day: 0, year: 1165,
                                          week: Some(0), week_day: super::Day::Sweetmorn},
                                          super::convert(0, -1).unwrap());
        assert_eq!(super::DiscordianDate {season: super::Season::Chaos,
                                          day: 0, year_day: 0, year: 0,
                                          week: Some(0), week_day: super::Day::Sweetmorn},
                                          super::convert(0, -1166).unwrap());
        assert_eq!(super::DiscordianDate {season: super::Season::Chaos,
                                          day: 0, year_day: 0, year: -1,
                                          week: Some(0), week_day: super::Day::Sweetmorn},
                                          super::convert(0, -1167).unwrap());
        assert_eq!(super::DiscordianDate {season: super::Season::StTibsDay,
                                          day: 0, year_day: 59, year: 3166,
                                          week: None, week_day: super::Day::StTibsDay},
                                          super::convert(59, 2000).unwrap());
        assert_eq!(super::DiscordianDate {season: super::Season::Chaos,
                                          day: 59, year_day: 60, year: 3166,
                                          week: Some(11), week_day: super::Day::SettingOrange},
                                          super::convert(60, 2000).unwrap());
        assert_eq!(super::DiscordianDate {season: super::Season::Discord,
                                          day: 11, year_day: 85, year: 3166,
                                          week: Some(16), week_day: super::Day::SettingOrange},
                                          super::convert(85, 2000).unwrap());
        assert_eq!(super::DiscordianDate {season: super::Season::TheAftermath,
                                          day: 72, year_day: 365, year: 3166,
                                          week: Some(72), week_day: super::Day::SettingOrange},
                                          super::convert(365, 2000).unwrap());
    }


    #[test]
    fn test_week_day() {
        assert_eq!(super::week_day(0), super::Day::Sweetmorn);
        assert_eq!(super::week_day(1), super::Day::Boomtime);
        assert_eq!(super::week_day(2), super::Day::Pungenday);
        assert_eq!(super::week_day(3), super::Day::PricklePrickle);
        assert_eq!(super::week_day(4), super::Day::SettingOrange);
        assert_eq!(super::week_day(10), super::Day::Sweetmorn);
        assert_eq!(super::week_day(12), super::Day::Pungenday);
        assert_eq!(super::week_day(21), super::Day::Boomtime);
    }

    #[test]
    fn test_leap_year_positive() {
        assert!(super::is_leap_year(2004));
        assert!(super::is_leap_year(2008));
        assert!(super::is_leap_year(2012));
        assert!(super::is_leap_year(2016));
    }

    #[test]
    fn test_leap_year_century() {
        assert!(super::is_leap_year(2000));
        assert!(!super::is_leap_year(1900));
        assert!(!super::is_leap_year(1800));
        assert!(!super::is_leap_year(2100));
    }

    #[test]
    fn test_leap_year_negative() {
        assert!(!super::is_leap_year(1998));
        assert!(!super::is_leap_year(1999));
        assert!(!super::is_leap_year(2014));
        assert!(!super::is_leap_year(2015));
    }
}
