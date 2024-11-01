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


pub struct Navbar {
    pub links: Vec<NavbarLink>,
}
impl Navbar {
    pub fn new(links: Vec<NavbarLink>) -> Self {
        Self { links }
    }
}
impl std::ops::Deref for Navbar {
    type Target = Vec<NavbarLink>;
    fn deref(&self) -> &Self::Target {
        &self.links
    }
}

#[derive(Deserialize)]
pub struct NavbarLink {
    pub disp: String,
    pub link: String,
}