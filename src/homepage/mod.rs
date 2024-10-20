// --------------------------------------------------
// mods
// --------------------------------------------------
pub mod tabs;

// this is required so that during static-site generation, can use: 
// 
// * `TabBodyType::AboutMe`
// 
// instead of
// 
// * `crate::homepage::tabs::TabBodyType::AboutMe`
// 
// inside of the template file of interest (in this case, `templates/homepage/index.html`)
//
//  vvvvvvvvvvvvvvvvv
use tabs::TabBodyType;

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