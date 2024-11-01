// --------------------------------------------------
// mods
// --------------------------------------------------
mod macros;
mod general;
mod homepage;
mod projects;
mod pathpattern;
pub(crate) mod prelude;

// --------------------------------------------------
// constants / statics
// --------------------------------------------------
pub static DEPLOY_DIR: OnceLock<PathBuf> = OnceLock::new();
pub const TEMPLATES_DIR: &'static str = concat!( env!("CARGO_MANIFEST_DIR"), "/templates" );
pub const DEPLOYMENT_MAP: &'static str = concat!( env!("CARGO_MANIFEST_DIR"), "/templates/deployment-map.json" );
pub const PROJECT_CATEGORIES_DIR: &'static str = concat!( env!("CARGO_MANIFEST_DIR"), "/templates/projects/categories" );

// --------------------------------------------------
// prelude
// --------------------------------------------------
pub use prelude::*;

// --------------------------------------------------
// external
// --------------------------------------------------
use std::{path::PathBuf, sync::OnceLock};

fn main() {
    // --------------------------------------------------
    // parse cli
    // --------------------------------------------------
    parse_cli();


    // for (n, ps) in projects::ALL_PROJECTS.iter() {
    //     println!("{:?}\n", n);   
    //     for p in ps {
    //         println!("{:?}\n\n", p);   
    //     }
    // }

    // let personal_projects = projects::ALL_PROJECTS.get("personal").unwrap();
    // let first_project = personal_projects.get(0).unwrap();
    // let ctx = first_project.render().unwrap();
    // println!("{}", ctx);

    println!("mapping: \n{:?}", *pathpattern::DEPLOYMENT_MAP);

    let ctx = homepage::HomePageTemplate::create();
    // println!("{}", ctx.render().unwrap());
}

/// Quick CLI: only input is required `--deploy <folder>`
fn parse_cli() {
    let args: Vec<String> = std::env::args().collect();
    let deploy_folder_arg = match args.iter().position(|arg| arg == "--deploy") {
        Some(pos) => pos,
        None => match args.iter().position(|arg| arg == "-d") {
            Some(pos) => pos,
            None => panic!("Missing `-d / --deploy` argument"),
        },
    };
    if deploy_folder_arg + 1 >= args.len() { panic!("Missing `-d / --deploy` argument") }
    let deploymen_dir = PathBuf::from(&args[deploy_folder_arg + 1]);
    // --------------------------------------------------
    // set statics
    // --------------------------------------------------
    DEPLOY_DIR.set(deploymen_dir).unwrap();
}