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
use std::path::PathBuf;

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

/// Convert MD -> HTML using `pandoc`
pub fn md2html(md_src_path: &PathBuf, filename: &str) -> String {
    let output = String::from_utf8_lossy(&std::process::Command::new("pandoc")
        .arg(&md_src_path)
        .arg("--to")
        .arg("html")
        .arg("--mathjax")
        .arg("-s")
        .arg("--strip-comments")
        // ----------------------------------------------------
        // <<STYLE+TAG>>
        // ----------------------------------------------------
        // This is included here and not in `templates/markdown.html`
        // ----------------------------------------------------
        .arg("--css")
        .arg("/css/std.css")
        .arg("--highlight-style=zenburn")
        .output()
        .expect(format!("Failed to run `pandoc` for note `{}`", filename).as_str())
        .stdout
    ).to_string();
    // ----------------------------------------------------
    // * remove everything up until `<style>` (this include `<!DOCTYPE html>` and `<head>` and `<meta>`)
    // * then, add back `<head>`
    // * then, remove the trailing `</html>`
    // ----------------------------------------------------
    // this is to ensure that the formatting is consistent
    // across all notes
    // ----------------------------------------------------
    // <<STYLE+TAG>>
    // ----------------------------------------------------
    format!("<head>{}", output[output.find("<style>").unwrap_or(0)..].trim_end_matches("</html>"))
}