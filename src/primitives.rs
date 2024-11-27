#[derive(Clone, Debug)]
/// The type of sidebar, and the contents to display
pub enum SidebarType {
    Projects,
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
pub struct Page<T> where T: askama::Template {
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