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
pub struct HomePageTemplate {
    pub tabs: Vec<tabs::Tab>,
}
/// [`HomePageTemplate`] implmentation of [`Create`]
impl Create for HomePageTemplate {
    fn create() -> Self {
        Self {
            tabs: (*tabs::ALL_TABS).clone(),
        }
    }
}