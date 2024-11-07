#[derive(Clone, Debug)]
/// The type of sidebar, and the contents to display
pub enum SidebarType {
    Projects,
    GatorOnly,
}

/// A page type
pub struct Page<T> where T: askama::Template {
    /// Indicates the path of the source file
    pub src: std::path::PathBuf,
    /// Any type required to construct the page
    pub page: T,
}