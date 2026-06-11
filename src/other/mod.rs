// --------------------------------------------------
// mods
// --------------------------------------------------
pub mod ideas;
pub mod lol;

// --------------------------------------------------
// local
// --------------------------------------------------
use crate::prelude::*;

// --------------------------------------------------
// re-exports
// --------------------------------------------------
pub use ideas::IdeasPage;
pub use lol::LolPage;

// --------------------------------------------------
// statics
// --------------------------------------------------
pub static OTHER_LINKS: LazyLock<Vec<Link>> = crate::lazy_json_template!("other/links.json");

#[derive(Template, Default)]
#[template(path = "other/other-homepage.html")]
/// Template for the Other category landing page
pub struct OtherHomepage {
    title: String,
    pub sidebar: SidebarType,
    pub links: Vec<Link>,
}
/// [`OtherHomepage`] implementation of [`Create`]
impl Create for OtherHomepage {
    fn create() -> Self {
        Self {
            title: crate::title!("Other"),
            links: (*OTHER_LINKS).clone(),
            ..Default::default()
        }
    }
}
/// [`OtherHomepage`] implementation of [`SourcePath`]
impl SourcePath<OtherHomepage> for OtherHomepage {
    fn src_path() -> std::path::PathBuf {
        [crate::TEMPLATES_DIR, "/other/other-homepage.html"]
            .concat()
            .into()
    }
}
