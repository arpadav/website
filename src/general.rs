#[derive(Clone, Debug)]
/// The type of sidebar, and the contents to display
pub enum SidebarType {
    Projects,
}

/// A page type, which every single one of my pages will be stored
/// as `(<src>, T)`, where:
/// 
/// * `src` indicates the path of the source file
/// * `T` is any type required to construct the page
pub type Page<T> = (std::path::PathBuf, T);