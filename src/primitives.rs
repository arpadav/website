/// Trait for truncating titles to a fixed visible width
/// suitable for the gator sidebar.
///
/// Implemented for `str` so that any title (project, post, etc.)
/// can call `.sidebar_title()` directly.
pub trait SidebarTitle {
    /// Maximum visible length (in chars) of a sidebar title.
    /// Longer titles get truncated with a trailing `...`
    /// (e.g. `2024 - some long project ti...`).
    const SIDEBAR_TITLE_MAXLEN: usize = 48;

    /// Returns the title truncated to fit [`SIDEBAR_TITLE_MAXLEN`]
    /// visible characters. Truncation counts by `char`s (not bytes)
    /// so multi-byte UTF-8 is preserved.
    fn sidebar_title(&self) -> String;
}
/// [`SidebarTitle`] implementation for [`str`]
impl SidebarTitle for str {
    fn sidebar_title(&self) -> String {
        let n = self.chars().count();
        if n <= Self::SIDEBAR_TITLE_MAXLEN {
            return self.to_string();
        }
        let keep = Self::SIDEBAR_TITLE_MAXLEN.saturating_sub(3);
        let mut out: String = self.chars().take(keep).collect();
        out.push_str("...");
        out
    }
}

#[derive(Clone, Debug)]
/// The type of sidebar, and the contents to display
pub enum SidebarType {
    Projects,
    Posts,
    GatorOnly,
}
/// [`SidebarType`] implmentation of [`Default`]
impl Default for SidebarType {
    fn default() -> Self {
        Self::GatorOnly
    }
}

#[derive(Clone, Debug)]
/// The source type, if multiple sources can
/// be generated into HTML
pub enum SourceType {
    Html,
    Markdown,
}

/// A page type
pub struct Page<T>
where
    T: askama::Template,
{
    /// Indicates the path of the source file
    pub src: std::path::PathBuf,
    /// Any type required to construct the page
    pub page: T,
}

#[derive(Clone, Debug)]
/// A link
pub struct Link {
    /// The name of the link
    pub name: String,
    /// The URL of the link
    pub url: String,
}

#[derive(Clone, Debug)]
/// A table-of-contents entry extracted from an HTML heading
pub struct TocEntry {
    /// Heading level (1-6)
    pub level: u8,
    /// The `id` attribute of the heading (used for anchor links)
    pub id: String,
    /// Plain-text content of the heading
    pub text: String,
}
