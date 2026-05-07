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
/// A parsed post date from `YYYY/MM/DD` directory components.
pub struct PostDateFormat {
    /// Year, e.g. `2026`
    pub year: u16,

    /// Month, 1-12
    pub month: u8,

    /// Day, 1-31
    pub day: u8,
}
/// [`PostDateFormat`] implementation
impl PostDateFormat {
    /// Build from the `YYYY`, `MM`, `DD` parent-directory strings.
    pub fn from_path_parts(year: &str, month: &str, day: &str) -> Result<Self, String> {
        if year.len() != 4 {
            return Err(format!("Year dir `{}` must be 4 digits", year));
        }
        if month.len() != 2 {
            return Err(format!("Month dir `{}` must be 2 digits", month));
        }
        if day.len() != 2 {
            return Err(format!("Day dir `{}` must be 2 digits", day));
        }
        let year: u16 = year
            .parse()
            .map_err(|_| format!("Invalid year `{}`", year))?;
        let month: u8 = month
            .parse()
            .map_err(|_| format!("Invalid month `{}`", month))?;
        let day: u8 = day.parse().map_err(|_| format!("Invalid day `{}`", day))?;
        if !(1..=12).contains(&month) {
            return Err(format!("Month {} out of range 1-12", month));
        }
        if !(1..=31).contains(&day) {
            return Err(format!("Day {} out of range 1-31", day));
        }
        Ok(Self { year, month, day })
    }

    /// Returns a sortable string `"YYYYMMDD"` for ordering posts.
    pub fn as_key(&self) -> String {
        format!("{:04}{:02}{:02}", self.year, self.month, self.day)
    }
}
/// [`PostDateFormat`] implementation of [`std::fmt::Display`]
impl std::fmt::Display for PostDateFormat {
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
