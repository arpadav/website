// --------------------------------------------------
// mods
// --------------------------------------------------
mod page;

// --------------------------------------------------
// local
// --------------------------------------------------
use crate::prelude::*;
use page::ProjectTemplate;

// --------------------------------------------------
// statics
// --------------------------------------------------
/// Parses the project directory using the following format:
///
/// <<STYLE+TAG>>>
/// 
/// ```no_run ignore
/// <templates-directory>/projects
/// ├── <category-0>
/// │   ├── <proj-0>
/// │   │   ├── proj-0.json
/// │   │   └── proj-0.html
/// │   ├── <proj-1>
/// │   │   ├── proj-1.json
/// │   │   └── proj-1.html
/// │   └── ...
/// ├── <category-1>
/// │   ├── <proj-0>
/// │   │   ├── proj-0.json
/// │   │   └── proj-0.html
/// │   └── ...
/// └── ...
/// ```
///
/// * Each `category` contains multiple projects
/// * Each project is expected to have both a `.json` and `.html`
/// file. If either file is missing, a warning is printed and the 
/// project is then skipped.
pub static ALL_PROJECTS: LazyLock<Vec<(String, Vec<Page<ProjectTemplate>>)>> = LazyLock::new(|| {
    // --------------------------------------------------
    // loop through project categories
    // --------------------------------------------------
    let mut pages: Vec<(String, Vec<Page<ProjectTemplate>>)> = std::fs::read_dir(crate::PROJECT_CATEGORIES_DIR)
        .ok()
        .into_iter()
        // --------------------------------------------------
        // handle invalid items pt 1
        // --------------------------------------------------
        .flat_map(|entries| entries.filter_map(Result::ok))
        // --------------------------------------------------
        // get the category name, pass it down
        // --------------------------------------------------
        .filter_map(|category_entry| {
            let category_path = category_entry.path();
            let category_name = category_path.file_name()?.to_string_lossy().to_string();
            let category_projects = std::fs::read_dir(category_path.clone()).ok()?;
            category_path.is_dir().then(|| (category_name, category_projects))
        })
        // --------------------------------------------------
        // handle invalid items pt 2
        // --------------------------------------------------
        .flat_map(|(category_name, projects)| projects
            .filter_map(Result::ok)
            .map(move |project_entry| (category_name.clone(), project_entry))
        )
        // --------------------------------------------------
        // check for project files, print warnings if incomplete
        // --------------------------------------------------
        .filter_map(|(category_name, project_entry)| {
            let project_path = project_entry.path();
            if !project_path.is_dir() { return None; }
            // --------------------------------------------------
            // get the name, header (.json) and template (.html)
            // --------------------------------------------------
            let project_name = project_path.file_name()?.to_string_lossy();
            let json_path = project_path.join(format!("{}.json", project_name));
            let html_path = project_path.join(format!("{}.html", project_name));
            // --------------------------------------------------
            // if doesnt exist, print warning
            // --------------------------------------------------
            if !json_path.exists() {
                eprintln!("Warning: Missing JSON file for project: {}", project_path.display());
                return None;
            }
            let json_path = json_path.display().to_string();
            if !html_path.exists() {
                eprintln!("Warning: Missing HTML file for project: {}", project_path.display());
                return None;
            }
            let content = std::fs::read_to_string(&html_path).ok()?;
            // --------------------------------------------------
            // return
            // --------------------------------------------------
            Some((
                category_name,
                html_path,
                ProjectTemplate {
                    name: project_name.to_string(),
                    // --------------------------------------------------
                    // <<STYLE+TAG>>
                    // --------------------------------------------------
                    url: format!("/projects/{}/", project_name),
                    header: crate::json_template!(json_path),
                    content,
                    sidebar: SidebarType::Projects,
                },
            ))
        })
        // --------------------------------------------------
        // put into hashmap
        // --------------------------------------------------
        .fold(HashMap::new(), |mut hm: HashMap<String, Vec<Page<ProjectTemplate>>>, (category, project_path, project_page)| {
            let project = Page { src: project_path.clone(), page: project_page };
            match hm.contains_key(&category) {
                true => hm.get_mut(&category).unwrap().push(project),
                false => { let _ = hm.insert(category.clone(), vec![project]); },
            };
            hm
        })
        // --------------------------------------------------
        // then, put into vec. this makes iteration manipulation (e.g. `rev`) easier
        // --------------------------------------------------
        .into_iter()
        .fold(Vec::new(), |mut v, (category, projects)| {
            v.push((category.clone(), projects));
            v
        });
    // --------------------------------------------------
    // sort projects based off category (reverse alphabetic)
    // then name (reverse alphabetic, which is actually reverse chronological
    // so that the most recent projects are first)
    // --------------------------------------------------
    // <<STYLE+TAG>>
    // --------------------------------------------------
    pages.sort_by(|a, b| b.0.cmp(&a.0));
    pages
        .iter_mut()
        .for_each(|(_, categorized_projects)|
        categorized_projects.sort_by(|a, b| b.page.name.cmp(&a.page.name))
    );
    // --------------------------------------------------
    // return
    // --------------------------------------------------
    pages
});

#[derive(Debug, Clone, Template)]
#[template(path = "projects/projects-homepage.html")]
/// Homepage which shows all projects
pub struct ProjectsHomepage {
    pub sidebar: SidebarType,
}
/// [`ProjectsHomepage`] implementation of [`Create`]
impl Create for ProjectsHomepage {
    fn create() -> Self {
        Self { sidebar: SidebarType::GatorOnly }
    }
}
/// [`ProjectsHomepage`] implementation of [`SourcePath`]
impl SourcePath<ProjectsHomepage> for ProjectsHomepage {
    fn src_path() -> std::path::PathBuf {
        [ crate::TEMPLATES_DIR, "/projects/projects-homepage.html" ].concat().into()
    }
}