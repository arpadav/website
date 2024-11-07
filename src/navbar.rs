// --------------------------------------------------
// external
// --------------------------------------------------
use serde::Deserialize;

// --------------------------------------------------
// local
// --------------------------------------------------
use crate::prelude::*;

// --------------------------------------------------
// statics
// --------------------------------------------------
pub static NAVBAR: LazyLock<Navbar> = LazyLock::new(|| {
    // --------------------------------------------------
    // read in json
    // --------------------------------------------------
    let navbar: Vec<NavbarLink> = crate::json_template!("general/navbar.json");
    Navbar::new(navbar)
});

/// A list of links in the navigation bar
pub struct Navbar {
    pub links: Vec<NavbarLink>,
}
/// [`Navbar`] implementation
impl Navbar {
    pub fn new(links: Vec<NavbarLink>) -> Self {
        Self { links }
    }
}
/// [`Navbar`] implementation of [`std::ops::Deref`]
impl std::ops::Deref for Navbar {
    type Target = Vec<NavbarLink>;
    fn deref(&self) -> &Self::Target {
        &self.links
    }
}

#[derive(Deserialize)]
/// A link in the navigation bar
pub struct NavbarLink {
    /// The name of the link being displayed
    pub disp: String,
    /// The URL of the link
    pub link: String,
}