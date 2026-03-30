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
/// A parsed blog date from a `YYYYMMDD-HHMM` filename prefix.
pub struct BlogDateFormat {
    /// Year, e.g. `2026`
    pub year: u16,

    /// Month, 1-12
    pub month: u8,

    /// Day, 1-31
    pub day: u8,

    /// Hour, 0-23
    pub hour: u8,

    /// Minute, 0-59
    pub minute: u8,
}
/// [`BlogDateFormat`] implementation
impl BlogDateFormat {
    /// Minimum filename length required: `YYYYMMDD-HHMM` = 13 characters.
    pub const PREFIX_LEN: usize = 13;

    /// Returns a sortable string `"YYYYMMDDHHMM"` for ordering posts.
    pub fn sort_key(&self) -> String {
        format!(
            "{:04}{:02}{:02}{:02}{:02}",
            self.year, self.month, self.day, self.hour, self.minute
        )
    }
}
/// [`BlogDateFormat`] implementation of [`std::fmt::Display`]
impl std::fmt::Display for BlogDateFormat {
    /// Renders as `"March 29, 2026"`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}, {}",
            MONTHS[(self.month - 1) as usize],
            self.day,
            self.year
        )
    }
}
/// [`BlogDateFormat`] implementation of [`std::str::FromStr`]
impl std::str::FromStr for BlogDateFormat {
    type Err = String;

    /// Parse from a filename starting with `YYYYMMDD-HHMM`.
    /// Extra characters after the prefix are ignored (e.g. `-my-post-title`).
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < Self::PREFIX_LEN {
            return Err(format!(
                "Blog filename `{}` too short — expected YYYYMMDD-HHMM prefix",
                s
            ));
        }
        if s.as_bytes()[8] != b'-' {
            return Err(format!(
                "Blog filename `{}` missing hyphen after YYYYMMDD",
                s
            ));
        }
        let year: u16 = s[..4]
            .parse()
            .map_err(|_| format!("Invalid year in blog filename `{}`", s))?;
        let month: u8 = s[4..6]
            .parse()
            .map_err(|_| format!("Invalid month in blog filename `{}`", s))?;
        let day: u8 = s[6..8]
            .parse()
            .map_err(|_| format!("Invalid day in blog filename `{}`", s))?;
        let hour: u8 = s[9..11]
            .parse()
            .map_err(|_| format!("Invalid hour in blog filename `{}`", s))?;
        let minute: u8 = s[11..13]
            .parse()
            .map_err(|_| format!("Invalid minute in blog filename `{}`", s))?;
        if !(1..=12).contains(&month) {
            return Err(format!("Month out of range in blog filename `{}`", s));
        }
        if !(1..=31).contains(&day) {
            return Err(format!("Day out of range in blog filename `{}`", s));
        }
        Ok(Self {
            year,
            month,
            day,
            hour,
            minute,
        })
    }
}
