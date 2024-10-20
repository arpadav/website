// --------------------------------------------------
// mods
// --------------------------------------------------
mod macros;
mod homepage;
mod projects;
pub(crate) mod prelude;

// --------------------------------------------------
// constants
// --------------------------------------------------
pub const TEMPLATES_DIR: &'static str = concat!( env!("CARGO_MANIFEST_DIR"), "/templates" );

// --------------------------------------------------
// prelude
// --------------------------------------------------
pub use prelude::*;

fn main() {
    let ctx = homepage::HomePageTemplate::create();
    println!("{}", ctx.render().unwrap());
}