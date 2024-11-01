// --------------------------------------------------
// mods
// --------------------------------------------------
mod macros;
mod navbar;
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
pub const DEPLOYMENT_MAP_JSON: &'static str = concat!( env!("CARGO_MANIFEST_DIR"), "/templates/deployment-map.json" );
pub const PROJECT_CATEGORIES_DIR: &'static str = concat!( env!("CARGO_MANIFEST_DIR"), "/templates/projects/categories" );

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
    for mapping in pathpattern::DEPLOYMENT_MAP.r().iter() {
        println!("{} -> {}", mapping.src.display(), mapping.dst.display());
    }

    // --------------------------------------------------
    // * get homepage. verify existance in deployment map
    // --------------------------------------------------
    let homepage = homepage::HomePageTemplate::to_page();
    assert!(pathpattern::DEPLOYMENT_MAP.r().exists(pathpattern::DeploymentFileType::Source(&homepage.0)), "homepage not found in deployment map");
    // --------------------------------------------------
    // * get projects homepage. verify existance in deployment map
    // --------------------------------------------------
    let phomepage = projects::ProjectHomePageTemplate::to_page();
    assert!(pathpattern::DEPLOYMENT_MAP.r().exists(pathpattern::DeploymentFileType::Source(&phomepage.0)), "projects homepage not found in deployment map");
    // --------------------------------------------------
    // * get project pages. verify existance in deployment map
    // --------------------------------------------------
    let projects = projects::ALL_PROJECTS.iter().map(|(path, (_, proj))| (path.clone(), proj.clone())).collect::<Vec<_>>();
    projects
        .iter()
        .for_each(|(ppath, _)| match pathpattern::DEPLOYMENT_MAP.r().exists(pathpattern::DeploymentFileType::Source(&ppath)) {
            true => (),
            false => panic!("{} not found in deployment map", ppath.display()),
        });

    // --------------------------------------------------
    // * render + deploy homepage
    // --------------------------------------------------
    deploy!(homepage, "homepage");
    // --------------------------------------------------
    // * render + deploy projects homepage
    // --------------------------------------------------
    deploy!(phomepage, "projects homepage");
    // --------------------------------------------------
    // * render + deploy project pages
    // --------------------------------------------------
    projects
        .into_iter()
        .for_each(|projectpage| {
            let name = projectpage.1.name.clone();
            deploy!(projectpage, &name);
        });
    // --------------------------------------------------
    // * render + deploy everything else
    // --------------------------------------------------
    pathpattern::DEPLOYMENT_MAP
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