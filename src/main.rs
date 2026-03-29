// --------------------------------------------------
// mods
// --------------------------------------------------
mod blog;
mod deployutil;
mod homepage;
mod macros;
mod miscpages;
mod navbar;
mod notes;
pub(crate) mod prelude;
mod primitives;
mod projects;

// --------------------------------------------------
// constants / statics
// --------------------------------------------------
pub static DEPLOY_DIR: OnceLock<PathBuf> = OnceLock::new();
pub const TEMPLATES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/templates");
pub const DEPLOYMENT_MAP_JSON: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/templates/deployment-map.json");
pub const PROJECT_CATEGORIES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/content/projects/");
pub const NOTES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/content/notes");
pub const BLOG_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/content/blog");

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
    deploy!(blog, blog::BlogHomepage);
    deploy!(gator, miscpages::Alligator);
    deploy!(language, miscpages::Language);

    // --------------------------------------------------
    // * get project pages. verify existance in deployment map
    // --------------------------------------------------
    // this is a pre-requisite for deploying. need to know
    // WHERE to deploy. the source is indicated by a `Page::src`
    // --------------------------------------------------
    let projects = projects::ALL_PROJECTS
        .iter()
        .flat_map(|(_, proj)| proj)
        .collect::<Vec<_>>();
    projects.iter().for_each(|proj| {
        match deployutil::DEPLOYMENT_MAP
            .r()
            .exists(deployutil::DeploymentFileType::Source(&proj.src))
        {
            true => (),
            false => panic!(
                "`{}` found in `projects`, but not found in deployment map",
                proj.src.display()
            ),
        }
    });
    // --------------------------------------------------
    // * render + deploy project pages
    // --------------------------------------------------
    projects.into_iter().for_each(|proj| {
        let name = proj.page.name.clone();
        page_deploy!(proj, &name);
    });

    // --------------------------------------------------
    // * get notes pages. verify existance in deployment map
    // --------------------------------------------------
    // this is a pre-requisite for deploying. need to know
    // WHERE to deploy. the source is indicated by a `Page::src`
    // --------------------------------------------------
    notes::NOTES.iter().for_each(|note| {
        match deployutil::DEPLOYMENT_MAP
            .r()
            .exists(deployutil::DeploymentFileType::Source(&note.src))
        {
            true => (),
            false => panic!(
                "`{}` found in `notes`, but not found in deployment map",
                note.src.display()
            ),
        }
    });
    // --------------------------------------------------
    // * render + deploy project pages
    // --------------------------------------------------
    notes::NOTES.iter().for_each(|note| {
        let name = note.src.display().to_string();
        page_deploy!(note, &name);
    });

    // --------------------------------------------------
    // * get blog pages. verify existance in deployment map
    // --------------------------------------------------
    blog::BLOG_PAGES.iter().for_each(|post| {
        match deployutil::DEPLOYMENT_MAP
            .r()
            .exists(deployutil::DeploymentFileType::Source(&post.src))
        {
            true => (),
            false => panic!(
                "`{}` found in `blog`, but not found in deployment map",
                post.src.display()
            ),
        }
    });
    // --------------------------------------------------
    // * render + deploy blog pages
    // --------------------------------------------------
    blog::BLOG_PAGES.iter().for_each(|post| {
        let name = post.src.display().to_string();
        page_deploy!(post, &name);
    });

    // --------------------------------------------------
    // * render + deploy everything else
    // --------------------------------------------------
    deployutil::DEPLOYMENT_MAP
        .w()
        .not_deployed()
        .for_each(|file| match file.copy() {
            Ok(_) => (),
            Err(e) => panic!(
                "Failed to copy {} to {}: {}",
                file.src.display(),
                file.dst.display(),
                e
            ),
        });
}

/// Quick CLI: only input is required `--deploy <folder>`
fn parse_cli() {
    let args: Vec<String> = std::env::args().collect();
    let deploy_folder_arg = match args.iter().position(|arg| arg == "--deploy") {
        Some(pos) => pos,
        None => match args.iter().position(|arg| arg == "-d") {
            Some(pos) => pos,
            None => panic!("Missing `-d / --deploy` argument. Usage: `-d / --deploy <folder>`"),
        },
    };
    if deploy_folder_arg + 1 >= args.len() {
        panic!("Missing `-d / --deploy` argument. Usage: `-d / --deploy <folder>`")
    }
    let deploymen_dir = PathBuf::from(&args[deploy_folder_arg + 1]);
    // --------------------------------------------------
    // set statics
    // --------------------------------------------------
    DEPLOY_DIR.set(deploymen_dir).unwrap();
}
