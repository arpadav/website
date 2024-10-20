// // --------------------------------------------------
// // mods
// // --------------------------------------------------
// pub mod tabs;

// // this is required so that during static-site generation, can use: 
// // 
// // * `TabBodyType::AboutMe`
// // 
// // instead of
// // 
// // * `crate::homepage::tabs::TabBodyType::AboutMe`
// // 
// // inside of the template file of interest (in this case, `templates/homepage/index.html`)
// //
// //  vvvvvvvvvvvvvvvvv
// use tabs::TabBodyType;

// --------------------------------------------------
// local
// --------------------------------------------------
use crate::prelude::*;

#[derive(Template)]
#[template(path = "projects/index.html")]
/// Template for projects page on my website!
pub struct ProjectPageTemplate {
    pub projects: Vec<i32>,
}
/// [`ProjectPageTemplate`] implmentation of [`Create`]
impl Create for ProjectPageTemplate {
    fn create() -> Self {
        Self {
            projects: vec![1, 2, 3],
        }
    }
}