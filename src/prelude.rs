// --------------------------------------------------
// external
// --------------------------------------------------
pub use askama::Template;

/// [`Render`] trait, for rendering custom HTML (safe) from elements
pub trait Render {
    fn render(&self) -> String;
}

/// [`Create`] trait, for generating static elements for templating
pub trait Create {
    fn create() -> Self;
}