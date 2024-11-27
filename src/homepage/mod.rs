// --------------------------------------------------
// mods
// --------------------------------------------------
pub mod tabs;

// --------------------------------------------------
// local
// --------------------------------------------------
use crate::prelude::*;

#[derive(Template)]
#[template(path = "homepage/index.html")]
/// Template for home / main page of my website!
pub struct LandingPage {
    title: String,
    pub tabs: Vec<tabs::Tab>,
}
/// [`LandingPage`] implmentation of [`Create`]
impl Create for LandingPage {
    fn create() -> Self {
        Self {
            title: crate::title!(),
            tabs: (*tabs::ALL_TABS).clone(),
        }
    }
}
/// [`LandingPage`] implmentation of [`SourcePath`]
impl SourcePath<LandingPage> for LandingPage {
    fn src_path() -> std::path::PathBuf {
        [ crate::TEMPLATES_DIR, "/homepage/index.html" ].concat().into()
    }
}