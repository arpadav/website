// --------------------------------------------------
// local
// --------------------------------------------------
use crate::prelude::*;
use crate::primitives::{Csv, CsvRow};

// --------------------------------------------------
// statics
// --------------------------------------------------
pub static IDEAS_ENTRIES: LazyLock<Vec<IdeaEntry>> = LazyLock::new(|| {
    let result = Csv::new(
        std::path::Path::new(crate::OTHER_DIR).join("ideas.csv"),
        ["date", "name", "description"],
    )
    .read_rows()
    .and_then(|rows| rows.into_iter().map(IdeaEntry::try_from).collect());
    // --------------------------------------------------
    // failed parse - either csv or code is malformed
    // --------------------------------------------------
    let mut entries: Vec<IdeaEntry> =
        result.unwrap_or_else(|e| panic!("Failed to parse idea entries: {e}"));
    // --------------------------------------------------
    // reverse chronological
    // --------------------------------------------------
    entries.sort_by(|a, b| {
        b.date
            .as_key()
            .cmp(&a.date.as_key())
            .then_with(|| a.name.cmp(&b.name))
    });
    entries
});

#[derive(Clone, Debug)]
/// One idea entry
pub struct IdeaEntry {
    pub date: DateFormat,
    pub name: String,
    pub description: String,
}
/// [`IdeaEntry`] implementation of [`TryFrom`] for [`CsvRow`]
impl TryFrom<CsvRow> for IdeaEntry {
    type Error = String;

    fn try_from(row: CsvRow) -> Result<Self, Self::Error> {
        let [date, name, description] = row.into_array("idea entry")?;
        Ok(Self {
            date: DateFormat::parse_csv(&date)?,
            description: MarkdownDocument::from_inline(&description, &name)?.html,
            name,
        })
    }
}

#[derive(Template, Default)]
#[template(path = "other/ideas.html")]
/// Template for the ideas page
pub struct IdeasPage {
    title: String,
    pub sidebar: SidebarType,
    pub entries: Vec<IdeaEntry>,
}
/// [`IdeasPage`] implementation of [`Create`]
impl Create for IdeasPage {
    fn create() -> Self {
        Self {
            title: crate::title!("Ideas"),
            sidebar: SidebarType::Other,
            entries: (*IDEAS_ENTRIES).clone(),
        }
    }
}
/// [`IdeasPage`] implementation of [`SourcePath`]
impl SourcePath<IdeasPage> for IdeasPage {
    fn src_path() -> std::path::PathBuf {
        [crate::TEMPLATES_DIR, "/other/ideas.html"].concat().into()
    }
}
