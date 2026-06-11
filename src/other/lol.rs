// --------------------------------------------------
// local
// --------------------------------------------------
use crate::prelude::*;
use crate::primitives::{Csv, CsvRow};

// --------------------------------------------------
// statics
// --------------------------------------------------
pub static LOL_ENTRIES: LazyLock<Vec<LolEntry>> = LazyLock::new(|| {
    let result = Csv::new(
        std::path::Path::new(crate::OTHER_DIR).join("lol.csv"),
        ["date", "url", "title"],
    )
    .read_rows()
    .and_then(|rows| rows.into_iter().map(LolEntry::try_from).collect());
    // --------------------------------------------------
    // failed parse - either csv or code is malformed
    // --------------------------------------------------
    let mut entries: Vec<LolEntry> =
        result.unwrap_or_else(|e| panic!("Failed to parse lol entries: {e}"));
    // --------------------------------------------------
    // reverse chronological
    // --------------------------------------------------
    entries.sort_by(|a, b| {
        b.date
            .as_key()
            .cmp(&a.date.as_key())
            .then_with(|| a.url.cmp(&b.url))
    });
    entries
});

#[derive(Clone, Debug)]
/// One list-of-links entry.
pub struct LolEntry {
    pub date: DateFormat,
    pub url: String,
    pub title: String,
}
/// [`LolEntry`] implementation of [`TryFrom`] for [`CsvRow`]
impl TryFrom<CsvRow> for LolEntry {
    type Error = String;

    fn try_from(row: CsvRow) -> Result<Self, Self::Error> {
        let [date, url, title] = row.into_array("lol entry")?;
        Ok(Self {
            date: DateFormat::parse_csv(&date)?,
            url,
            title,
        })
    }
}

#[derive(Template, Default)]
#[template(path = "other/lol.html")]
/// Template for the lol (list-of-links) page
pub struct LolPage {
    title: String,
    pub sidebar: SidebarType,
    pub entries: Vec<LolEntry>,
}
/// [`LolPage`] implementation of [`Create`]
impl Create for LolPage {
    fn create() -> Self {
        Self {
            title: crate::title!("lol (list-of-links)"),
            sidebar: SidebarType::Other,
            entries: (*LOL_ENTRIES).clone(),
        }
    }
}
/// [`LolPage`] implementation of [`SourcePath`]
impl SourcePath<LolPage> for LolPage {
    fn src_path() -> std::path::PathBuf {
        [crate::TEMPLATES_DIR, "/other/lol.html"].concat().into()
    }
}
