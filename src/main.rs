// --------------------------------------------------
// mods
// --------------------------------------------------
mod macros;
mod general;
mod homepage;
mod projects;
pub(crate) mod prelude;

// --------------------------------------------------
// constants
// --------------------------------------------------
pub const TEMPLATES_DIR: &'static str = concat!( env!("CARGO_MANIFEST_DIR"), "/templates" );
pub const PROJECT_CATEGORIES_DIR: &'static str = concat!( env!("CARGO_MANIFEST_DIR"), "/templates/projects/categories" );

// --------------------------------------------------
// prelude
// --------------------------------------------------
pub use prelude::*;

fn main() {
    // for ps in projects::ALL_PROJECTS.values() {
    //     for p in ps {
    //         println!("{:?}\n\n", p.render().unwrap());   
    //     }
    // }

    // let personal_projects = projects::ALL_PROJECTS.get("personal").unwrap();
    // let first_project = personal_projects.get(0).unwrap();
    // let ctx = first_project.render().unwrap();
    // println!("{}", ctx);

    let ctx = homepage::HomePageTemplate::create();
    println!("{}", ctx.render().unwrap());
}