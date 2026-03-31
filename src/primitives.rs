#[derive(Clone, Debug)]
/// The type of sidebar, and the contents to display
pub enum SidebarType {
    Projects,
    Blog,
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
