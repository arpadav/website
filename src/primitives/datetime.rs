// --------------------------------------------------
// constants
// --------------------------------------------------
const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

#[derive(Clone, Debug)]
/// A date with optional hierarchical time components.
pub struct DateFormat {
    /// Calendar date.
    date: CalendarDate,
    /// Optional clock time.
    time: Option<ClockTime>,
}
/// [`DateFormat`] implementation
impl DateFormat {
    /// Build from the `YYYY`, `MM`, `DD` parent-directory strings.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use website::DateFormat;
    ///
    /// let date = DateFormat::from_path_parts("2026", "03", "29").unwrap();
    ///
    /// assert_eq!(date.as_key(), "20260329");
    /// assert_eq!(date.to_string(), "March 29, 2026");
    /// ```
    pub fn from_path_parts(year: &str, month: &str, day: &str) -> Result<Self, String> {
        // --------------------------------------------------
        // parse calendar date
        // --------------------------------------------------
        let date = CalendarDate::from_path_parts(year, month, day)?;
        // --------------------------------------------------
        // construct date-only format
        // --------------------------------------------------
        Ok(Self { date, time: None })
    }

    /// Build from `YYYY-MM-DD`, optionally followed by hierarchical time.
    ///
    /// Accepted examples:
    ///
    /// * `2026-06-11`
    /// * `2026-06-11 13`
    /// * `2026-06-11 13:45`
    /// * `2026-06-11 13:45:30`
    /// * `2026-06-11 13:45:30 -0400`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use website::DateFormat;
    ///
    /// let date = DateFormat::from_csv("2026-06-11 13:45:30 -0400");
    ///
    /// assert_eq!(date.as_key(), "20260611134530");
    /// assert_eq!(date.to_string(), "June 11, 2026 13:45:30 -0400");
    /// ```
    ///
    /// ```rust
    /// use website::DateFormat;
    ///
    /// let date = DateFormat::from_csv("2026-06-11 13:45");
    ///
    /// assert_eq!(date.as_key(), "202606111345");
    /// assert_eq!(date.to_string(), "June 11, 2026 13:45");
    /// ```
    ///
    /// ```should_panic
    /// use website::DateFormat;
    ///
    /// let _ = DateFormat::from_csv("2026-06-11 13:45 -0400");
    /// ```
    pub fn from_csv(input: &str) -> Self {
        Self::parse_csv(input).unwrap_or_else(|e| panic!("Invalid CSV date `{}`: {}", input, e))
    }

    /// Parse a CSV date string into a [`DateFormat`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use website::DateFormat;
    ///
    /// let date = DateFormat::parse_csv("2026-06-11").unwrap();
    ///
    /// assert_eq!(date.as_key(), "20260611");
    /// ```
    pub fn parse_csv(input: &str) -> Result<Self, String> {
        // --------------------------------------------------
        // split date from optional time
        // --------------------------------------------------
        let (date, time) = Self::split_csv_date(input);
        // --------------------------------------------------
        // parse calendar date
        // --------------------------------------------------
        let date = CalendarDate::parse_csv(date)?;
        // --------------------------------------------------
        // parse optional clock time
        // --------------------------------------------------
        let time = ClockTime::parse_optional(time)?;
        // --------------------------------------------------
        // construct date format
        // --------------------------------------------------
        Ok(Self { date, time })
    }

    /// Parse a four-digit year.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use website::DateFormat;
    ///
    /// assert_eq!(DateFormat::parse_year("2026").unwrap(), 2026);
    /// assert!(DateFormat::parse_year("26").is_err());
    /// ```
    pub fn parse_year(input: &str) -> Result<u16, String> {
        Year::parse(input).map(|year| year.value())
    }

    /// Parse a two-digit month.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use website::DateFormat;
    ///
    /// assert_eq!(DateFormat::parse_month("06").unwrap(), 6);
    /// assert!(DateFormat::parse_month("13").is_err());
    /// ```
    pub fn parse_month(input: &str) -> Result<u8, String> {
        Month::parse(input).map(|month| month.value())
    }

    /// Parse a two-digit day.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use website::DateFormat;
    ///
    /// assert_eq!(DateFormat::parse_day("11").unwrap(), 11);
    /// assert!(DateFormat::parse_day("32").is_err());
    /// ```
    pub fn parse_day(input: &str) -> Result<u8, String> {
        Day::parse(input).map(|day| day.value())
    }

    /// Parse a two-digit hour.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use website::DateFormat;
    ///
    /// assert_eq!(DateFormat::parse_hour("13").unwrap(), 13);
    /// assert!(DateFormat::parse_hour("24").is_err());
    /// ```
    pub fn parse_hour(input: &str) -> Result<u8, String> {
        Hour::parse(input).map(|hour| hour.value())
    }

    /// Parse a two-digit minute.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use website::DateFormat;
    ///
    /// assert_eq!(DateFormat::parse_minute("45").unwrap(), 45);
    /// assert!(DateFormat::parse_minute("60").is_err());
    /// ```
    pub fn parse_minute(input: &str) -> Result<u8, String> {
        Minute::parse(input).map(|minute| minute.value())
    }

    /// Parse a two-digit second.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use website::DateFormat;
    ///
    /// assert_eq!(DateFormat::parse_second("30").unwrap(), 30);
    /// assert!(DateFormat::parse_second("60").is_err());
    /// ```
    pub fn parse_second(input: &str) -> Result<u8, String> {
        Second::parse(input).map(|second| second.value())
    }

    /// Parse a `+/-ZZZZ` time zone offset.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use website::DateFormat;
    ///
    /// assert_eq!(DateFormat::parse_zone("-0400").unwrap(), "-0400");
    /// assert!(DateFormat::parse_zone("-2460").is_err());
    /// ```
    pub fn parse_zone(input: &str) -> Result<String, String> {
        Zone::parse(input).map(Zone::into_string)
    }

    /// Returns the year.
    pub fn year(&self) -> u16 {
        self.date.year()
    }

    /// Returns the month.
    pub fn month(&self) -> u8 {
        self.date.month()
    }

    /// Returns the day.
    pub fn day(&self) -> u8 {
        self.date.day()
    }

    /// Returns a sortable date/time key.
    pub fn as_key(&self) -> String {
        // --------------------------------------------------
        // start with calendar key
        // --------------------------------------------------
        let mut key = self.date.as_key();
        // --------------------------------------------------
        // append time key when present
        // --------------------------------------------------
        if let Some(time) = &self.time {
            time.push_key(&mut key);
        }
        // --------------------------------------------------
        // return sortable key
        // --------------------------------------------------
        key
    }

    /// Returns the calendar date label.
    pub fn date_label(&self) -> String {
        self.date.to_string()
    }

    /// Returns the clock time label.
    pub fn time_label(&self) -> String {
        self.time
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default()
    }

    /// Splits a CSV date from optional time details.
    fn split_csv_date(input: &str) -> (&str, Option<&str>) {
        // --------------------------------------------------
        // split on first space
        // --------------------------------------------------
        input
            .split_once(' ')
            .map_or((input, None), |(date, rest)| (date, Some(rest)))
    }
}
/// [`DateFormat`] implementation of [`std::fmt::Display`]
impl std::fmt::Display for DateFormat {
    /// Renders the date with whatever time precision exists.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // --------------------------------------------------
        // render calendar date
        // --------------------------------------------------
        write!(f, "{}", self.date)?;
        // --------------------------------------------------
        // render time when present
        // --------------------------------------------------
        if let Some(time) = &self.time {
            write!(f, " {time}")?;
        }
        // --------------------------------------------------
        // finish formatting
        // --------------------------------------------------
        Ok(())
    }
}

#[derive(Clone, Debug)]
/// A validated calendar date.
struct CalendarDate {
    /// Year component.
    year: Year,
    /// Month component.
    month: Month,
    /// Day component.
    day: Day,
}
/// [`CalendarDate`] implementation
impl CalendarDate {
    /// Builds a calendar date from path components.
    fn from_path_parts(year: &str, month: &str, day: &str) -> Result<Self, String> {
        // --------------------------------------------------
        // parse calendar components
        // --------------------------------------------------
        let year = Year::parse(year)?;
        let month = Month::parse(month)?;
        let day = Day::parse(day)?;
        // --------------------------------------------------
        // construct calendar date
        // --------------------------------------------------
        Ok(Self { year, month, day })
    }

    /// Builds a calendar date from a CSV date segment.
    fn parse_csv(date: &str) -> Result<Self, String> {
        // --------------------------------------------------
        // split calendar components
        // --------------------------------------------------
        let date_parts = date.split('-').collect::<Vec<_>>();
        if date_parts.len() != 3 {
            return Err("Date must be YYYY-MM-DD".to_string());
        }
        // --------------------------------------------------
        // parse calendar date
        // --------------------------------------------------
        Self::from_path_parts(date_parts[0], date_parts[1], date_parts[2])
    }

    /// Returns the year.
    fn year(&self) -> u16 {
        self.year.value()
    }

    /// Returns the month.
    fn month(&self) -> u8 {
        self.month.value()
    }

    /// Returns the day.
    fn day(&self) -> u8 {
        self.day.value()
    }

    /// Returns a sortable calendar key.
    fn as_key(&self) -> String {
        format!("{:04}{:02}{:02}", self.year(), self.month(), self.day())
    }
}
/// [`CalendarDate`] implementation of [`std::fmt::Display`]
impl std::fmt::Display for CalendarDate {
    /// Renders the calendar date.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}, {}",
            MONTHS[(self.month() - 1) as usize],
            self.day(),
            self.year()
        )
    }
}

#[derive(Clone, Debug)]
/// A validated clock time with hierarchical precision.
enum ClockTime {
    /// Hour precision.
    Hour(Hour),
    /// Minute precision.
    Minute(Hour, Minute),
    /// Second precision.
    Second(Hour, Minute, Second),
    /// Second precision with a time zone.
    Zoned(Hour, Minute, Second, Zone),
}
/// [`ClockTime`] implementation
impl ClockTime {
    /// Parses optional CSV time details.
    fn parse_optional(input: Option<&str>) -> Result<Option<Self>, String> {
        // --------------------------------------------------
        // return no time when absent
        // --------------------------------------------------
        let Some(input) = input else {
            return Ok(None);
        };
        // --------------------------------------------------
        // parse present time
        // --------------------------------------------------
        Self::parse_time_and_zone(input).map(Some)
    }

    /// Parses a time segment with an optional zone.
    fn parse_time_and_zone(input: &str) -> Result<Self, String> {
        // --------------------------------------------------
        // split clock time from optional zone
        // --------------------------------------------------
        let mut parts = input.split(' ');
        let time = parts
            .next()
            .filter(|time| !time.is_empty())
            .ok_or_else(|| "Time must be HH[:MM[:SS]] with optional zone".to_string())?;
        let zone = parts.next();
        if parts.next().is_some() || zone == Some("") {
            return Err("Time must be HH[:MM[:SS]] with optional zone".to_string());
        }
        // --------------------------------------------------
        // parse clock precision
        // --------------------------------------------------
        let time = Self::parse_clock(time)?;
        // --------------------------------------------------
        // add zone when present
        // --------------------------------------------------
        match zone {
            Some(zone) => time.with_zone(Zone::parse(zone)?),
            None => Ok(time),
        }
    }

    /// Parses an `HH[:MM[:SS]]` clock segment.
    fn parse_clock(time: &str) -> Result<Self, String> {
        // --------------------------------------------------
        // split time components
        // --------------------------------------------------
        let time_parts = time.split(':').collect::<Vec<_>>();
        if time_parts.is_empty() || time_parts.len() > 3 || time_parts.iter().any(|x| x.len() != 2)
        {
            return Err("Time must be HH[:MM[:SS]]".to_string());
        }
        // --------------------------------------------------
        // parse required hour
        // --------------------------------------------------
        let hour = Hour::parse(time_parts[0])?;
        // --------------------------------------------------
        // construct matching precision
        // --------------------------------------------------
        match time_parts.len() {
            1 => Ok(Self::Hour(hour)),
            2 => Ok(Self::Minute(hour, Minute::parse(time_parts[1])?)),
            3 => Ok(Self::Second(
                hour,
                Minute::parse(time_parts[1])?,
                Second::parse(time_parts[2])?,
            )),
            _ => Err("Time must be HH[:MM[:SS]]".to_string()),
        }
    }

    /// Adds a time zone to a second-precision clock time.
    fn with_zone(self, zone: Zone) -> Result<Self, String> {
        // --------------------------------------------------
        // require second precision before zone
        // --------------------------------------------------
        match self {
            Self::Second(hour, minute, second) => Ok(Self::Zoned(hour, minute, second, zone)),
            Self::Zoned(..) => Ok(self),
            Self::Hour(_) | Self::Minute(_, _) => {
                Err("Time zone cannot exist without second".to_string())
            }
        }
    }

    /// Appends the clock portion to a sortable key.
    fn push_key(&self, key: &mut String) {
        // --------------------------------------------------
        // append components by precision
        // --------------------------------------------------
        match self {
            Self::Hour(hour) => {
                hour.push_key(key);
            }
            Self::Minute(hour, minute) => {
                hour.push_key(key);
                minute.push_key(key);
            }
            Self::Second(hour, minute, second) | Self::Zoned(hour, minute, second, _) => {
                hour.push_key(key);
                minute.push_key(key);
                second.push_key(key);
            }
        }
    }
}
/// [`ClockTime`] implementation of [`std::fmt::Display`]
impl std::fmt::Display for ClockTime {
    /// Renders the clock time.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hour(hour) => write!(f, "{hour}"),
            Self::Minute(hour, minute) => write!(f, "{hour}:{minute}"),
            Self::Second(hour, minute, second) => write!(f, "{hour}:{minute}:{second}"),
            Self::Zoned(hour, minute, second, zone) => {
                write!(f, "{hour}:{minute}:{second} {zone}")
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
/// A validated year.
struct Year(u16);
/// [`Year`] implementation
impl Year {
    /// Parses a year.
    fn parse(input: &str) -> Result<Self, String> {
        NumericComponent::parse_u16(input, "year", 4).map(Self)
    }

    /// Returns the year.
    fn value(&self) -> u16 {
        self.0
    }
}

#[derive(Clone, Copy, Debug)]
/// A validated month.
struct Month(u8);
/// [`Month`] implementation
impl Month {
    /// Parses a month.
    fn parse(input: &str) -> Result<Self, String> {
        // --------------------------------------------------
        // parse numeric value
        // --------------------------------------------------
        let month = NumericComponent::parse_u8(input, "month", 2)?;
        // --------------------------------------------------
        // validate range
        // --------------------------------------------------
        NumericComponent::validate_u8_range(month, "Month", 1, 12)?;
        Ok(Self(month))
    }

    /// Returns the month.
    fn value(&self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy, Debug)]
/// A validated day.
struct Day(u8);
/// [`Day`] implementation
impl Day {
    /// Parses a day.
    fn parse(input: &str) -> Result<Self, String> {
        // --------------------------------------------------
        // parse numeric value
        // --------------------------------------------------
        let day = NumericComponent::parse_u8(input, "day", 2)?;
        // --------------------------------------------------
        // validate range
        // --------------------------------------------------
        NumericComponent::validate_u8_range(day, "Day", 1, 31)?;
        Ok(Self(day))
    }

    /// Returns the day.
    fn value(&self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy, Debug)]
/// A validated hour.
struct Hour(u8);
/// [`Hour`] implementation
impl Hour {
    /// Parses an hour.
    fn parse(input: &str) -> Result<Self, String> {
        // --------------------------------------------------
        // parse numeric value
        // --------------------------------------------------
        let hour = NumericComponent::parse_u8(input, "hour", 2)?;
        // --------------------------------------------------
        // validate range
        // --------------------------------------------------
        NumericComponent::validate_u8_range(hour, "Hour", 0, 23)?;
        Ok(Self(hour))
    }

    /// Returns the hour.
    fn value(&self) -> u8 {
        self.0
    }

    /// Appends the hour to a sortable key.
    fn push_key(&self, key: &mut String) {
        key.push_str(&format!("{:02}", self.value()));
    }
}
/// [`Hour`] implementation of [`std::fmt::Display`]
impl std::fmt::Display for Hour {
    /// Renders the hour.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.value())
    }
}

#[derive(Clone, Copy, Debug)]
/// A validated minute.
struct Minute(u8);
/// [`Minute`] implementation
impl Minute {
    /// Parses a minute.
    fn parse(input: &str) -> Result<Self, String> {
        // --------------------------------------------------
        // parse numeric value
        // --------------------------------------------------
        let minute = NumericComponent::parse_u8(input, "minute", 2)?;
        // --------------------------------------------------
        // validate range
        // --------------------------------------------------
        NumericComponent::validate_u8_range(minute, "Minute", 0, 59)?;
        Ok(Self(minute))
    }

    /// Returns the minute.
    fn value(&self) -> u8 {
        self.0
    }

    /// Appends the minute to a sortable key.
    fn push_key(&self, key: &mut String) {
        key.push_str(&format!("{:02}", self.value()));
    }
}
/// [`Minute`] implementation of [`std::fmt::Display`]
impl std::fmt::Display for Minute {
    /// Renders the minute.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.value())
    }
}

#[derive(Clone, Copy, Debug)]
/// A validated second.
struct Second(u8);
/// [`Second`] implementation
impl Second {
    /// Parses a second.
    fn parse(input: &str) -> Result<Self, String> {
        // --------------------------------------------------
        // parse numeric value
        // --------------------------------------------------
        let second = NumericComponent::parse_u8(input, "second", 2)?;
        // --------------------------------------------------
        // validate range
        // --------------------------------------------------
        NumericComponent::validate_u8_range(second, "Second", 0, 59)?;
        Ok(Self(second))
    }

    /// Returns the second.
    fn value(&self) -> u8 {
        self.0
    }

    /// Appends the second to a sortable key.
    fn push_key(&self, key: &mut String) {
        key.push_str(&format!("{:02}", self.value()));
    }
}
/// [`Second`] implementation of [`std::fmt::Display`]
impl std::fmt::Display for Second {
    /// Renders the second.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.value())
    }
}

#[derive(Clone, Debug)]
/// A validated time zone offset.
struct Zone(String);
/// [`Zone`] implementation
impl Zone {
    /// Parses a time zone offset.
    fn parse(input: &str) -> Result<Self, String> {
        // --------------------------------------------------
        // validate zone shape
        // --------------------------------------------------
        let bytes = input.as_bytes();
        if bytes.len() != 5 || !matches!(bytes[0], b'+' | b'-') {
            return Err(format!("Invalid time zone `{}`", input));
        }
        // --------------------------------------------------
        // parse zone hour
        // --------------------------------------------------
        let hour = NumericComponent::parse_u8(&input[1..3], "time zone hour", 2)
            .map_err(|_| format!("Invalid time zone `{}`", input))?;
        // --------------------------------------------------
        // validate zone hour
        // --------------------------------------------------
        NumericComponent::validate_u8_range(hour, "Time zone hour", 0, 23)?;
        // --------------------------------------------------
        // parse zone minute
        // --------------------------------------------------
        let minute = NumericComponent::parse_u8(&input[3..5], "time zone minute", 2)
            .map_err(|_| format!("Invalid time zone `{}`", input))?;
        // --------------------------------------------------
        // validate zone minute
        // --------------------------------------------------
        NumericComponent::validate_u8_range(minute, "Time zone minute", 0, 59)?;
        Ok(Self(input.to_string()))
    }

    /// Converts the zone into its string representation.
    fn into_string(self) -> String {
        self.0
    }
}
/// [`Zone`] implementation of [`std::fmt::Display`]
impl std::fmt::Display for Zone {
    /// Renders the zone.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Numeric component parsing helpers.
struct NumericComponent;
/// [`NumericComponent`] implementation
impl NumericComponent {
    /// Parses a fixed-width `u16`.
    fn parse_u16(input: &str, field: &str, width: usize) -> Result<u16, String> {
        // --------------------------------------------------
        // validate component width
        // --------------------------------------------------
        Self::validate_component_width(input, field, width)?;
        // --------------------------------------------------
        // parse component digits
        // --------------------------------------------------
        Self::parse_unsigned(input, field)
    }

    /// Parses a fixed-width `u8`.
    fn parse_u8(input: &str, field: &str, width: usize) -> Result<u8, String> {
        // --------------------------------------------------
        // validate component width
        // --------------------------------------------------
        Self::validate_component_width(input, field, width)?;
        // --------------------------------------------------
        // parse component digits
        // --------------------------------------------------
        Self::parse_unsigned(input, field)
    }

    /// Parses an unsigned numeric string.
    fn parse_unsigned<T>(input: &str, field: &str) -> Result<T, String>
    where
        T: std::str::FromStr,
    {
        // --------------------------------------------------
        // validate numeric characters
        // --------------------------------------------------
        if !input.bytes().all(|b| b.is_ascii_digit()) {
            return Err(format!("Invalid {} `{}`", field, input));
        }
        // --------------------------------------------------
        // parse numeric value
        // --------------------------------------------------
        input
            .parse()
            .map_err(|_| format!("Invalid {} `{}`", field, input))
    }

    /// Validates a component string width.
    fn validate_component_width(input: &str, field: &str, width: usize) -> Result<(), String> {
        // --------------------------------------------------
        // validate exact width
        // --------------------------------------------------
        if input.len() != width {
            return Err(format!("{} `{}` must be {} digits", field, input, width));
        }
        Ok(())
    }

    /// Validates a `u8` component range.
    fn validate_u8_range(value: u8, field: &str, min: u8, max: u8) -> Result<(), String> {
        // --------------------------------------------------
        // validate inclusive range
        // --------------------------------------------------
        (min..=max)
            .contains(&value)
            .then_some(())
            .ok_or_else(|| format!("{} {} out of range {}-{}", field, value, min, max))
    }
}
