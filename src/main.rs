// --------------------------------------------------
// mods
// --------------------------------------------------
mod notes;
mod macros;
mod navbar;
mod homepage;
mod projects;
mod miscpages;
mod deployutil;
mod primitives;
pub(crate) mod prelude;

// --------------------------------------------------
// constants / statics
// --------------------------------------------------
pub static DEPLOY_DIR: OnceLock<PathBuf> = OnceLock::new();
pub const TEMPLATES_DIR: &'static str = concat!( env!("CARGO_MANIFEST_DIR"), "/templates" );
pub const DEPLOYMENT_MAP_JSON: &'static str = concat!( env!("CARGO_MANIFEST_DIR"), "/templates/deployment-map.json" );
pub const PROJECT_CATEGORIES_DIR: &'static str = concat!( env!("CARGO_MANIFEST_DIR"), "/templates/projects/categories" );
pub const NOTES_DIR: &'static str = concat!( env!("CARGO_MANIFEST_DIR"), "/static/files/notes" );

// --------------------------------------------------
// prelude
// --------------------------------------------------
pub use prelude::*;

// --------------------------------------------------
// external
// --------------------------------------------------
use std::{
    path::PathBuf,
    sync::OnceLock,
};

fn main() {
    // --------------------------------------------------
    // parse cli
    // --------------------------------------------------
    parse_cli();

    // --------------------------------------------------
    // print deployment map
    // --------------------------------------------------
    for mapping in deployutil::DEPLOYMENT_MAP.r().iter() {
        println!("{} -> {}", mapping.src.display(), mapping.dst.display());
    }

    // --------------------------------------------------
    // 1. get the following individual pages.
    // 2. then, verify existence in deployment map
    // 3. then, render and deploy
    // --------------------------------------------------
    deploy!(landing_page, homepage::LandingPage);
    deploy!(projects, projects::ProjectsHomepage);
    deploy!(notes, notes::NotesHomepage);
    deploy!(gator, miscpages::Alligator);
    deploy!(language, miscpages::Language);

    // --------------------------------------------------
    // * get project pages. verify existance in deployment map
    // --------------------------------------------------
    let projects = projects::ALL_PROJECTS.iter().map(|(_, proj)| proj).flatten().collect::<Vec<_>>();
    projects
        .iter()
        .for_each(|proj| match deployutil::DEPLOYMENT_MAP.r().exists(deployutil::DeploymentFileType::Source(&proj.src)) {
            true => (),
            false => panic!("{} not found in deployment map", proj.src.display()),
        });
    // --------------------------------------------------
    // * render + deploy project pages
    // --------------------------------------------------
    projects
        .into_iter()
        .for_each(|proj| {
            let name = proj.page.name.clone();
            page_deploy!(proj, &name);
        });
    
    // --------------------------------------------------
    // * render + deploy everything else
    // --------------------------------------------------
    deployutil::DEPLOYMENT_MAP
        .w()
        .not_deployed()
        .for_each(|file| match file.copy() {
            Ok(_) => (),
            Err(e) => panic!("Failed to copy {} to {}: {}", file.src.display(), file.dst.display(), e),
        });
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