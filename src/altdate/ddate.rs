
#[derive(Debug,PartialEq)]
enum Day {
    Sweetmorn,
    Boomtime,
    Pungenday,
    PricklePrickle,
    SettingOrange,
    StTibsDay,
}

#[derive(Debug)]
enum Season {
    Chaos,
    Discord,
    Confusion,
    Bureaucracy,
    TheAftermath,
    StTibsDay,
}


#[derive(Debug)]
pub struct DiscordianDate {
    season: Season,
    day: u8,
    year_day: u16,
    year: i32,
    week_day: Day,
    week: Option<u8>,
}


pub fn convert(nday: u16, nyear: i32) -> Option<DiscordianDate> {
    let year = nyear + 1166;
    let year_day = nday + 1;  // Switch to one-based

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
        let day = ((nday % 73) + 1) as u8;
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
                59 => 1,
                60 ... 365 => (nday - 1) % 73 + 1,
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


fn is_leap_year(year_ce: i32) -> bool {
    let has_factor = |n| year_ce % n == 0;
    return has_factor(4) && !has_factor(100) || has_factor(400)
}

#[cfg(test)]
mod test {

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
