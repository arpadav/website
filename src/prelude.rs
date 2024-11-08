// --------------------------------------------------
// external
// --------------------------------------------------
// normally i would never do this (especially for 
// data-structures like LazyLock and HashMap)
// however, these 4 items are so ubiquitously used
// in this implementation that it is more convenient
// to include them all here
// --------------------------------------------------
pub use askama::Template;
pub use serde::Deserialize;
pub use std::sync::LazyLock;
pub use std::collections::HashMap;

// --------------------------------------------------
// local
// --------------------------------------------------
// similar to above; this is unusual to do. however:
//
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
// therefore, including this all into the prelude so that
// we dont have to forget to import it in every template
// to enable enum short-syntax in templates
// --------------------------------------------------
pub use crate::primitives::*;
pub use crate::homepage::tabs::TabBodyType;

/// [`Render`] trait, for rendering custom HTML (safe) from elements
pub trait Render {
    fn render(&self) -> String;
}

/// [`Create`] trait, for generating static elements for templating
pub trait Create {
    fn create() -> Self;
}

/// [`SourcePath`] trait, for setting the path of any type
pub trait SourcePath<T> {
    fn src_path() -> std::path::PathBuf;
}

/// [`ToPage`] trait, for creating a [`Page`] from any type
pub trait ToPage<T> where T: Create + SourcePath<T> + askama::Template {
    fn to_page() -> Page<T> {
        Page {
            src: T::src_path(),
            page: T::create(),
        }
    }
}
impl<T> ToPage<T> for T where T: Create + SourcePath<T> + askama::Template {}