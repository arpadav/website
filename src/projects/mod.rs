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
pub static ALL_PROJECTS: LazyLock<Vec<Page<(String, ProjectTemplate)>>> = LazyLock::new(|| {
    // --------------------------------------------------
    // loop through project categories
    // --------------------------------------------------
    let mut pages: Vec<Page<(String, ProjectTemplate)>> = std::fs::read_dir(crate::PROJECT_CATEGORIES_DIR)
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
            let pp = project_path.clone();
            let project_name = pp.file_name()?.to_string_lossy();
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
                project_path,
                category_name,
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
        .fold(Vec::new(), |mut map, (ppath, category, project)| {
            let page: Page<(String, ProjectTemplate)> = (ppath.clone(), (category.clone(), project));
            map.push(page);
            map
        });
    // --------------------------------------------------
    // sort projects based off category, then name
    // --------------------------------------------------
    pages.sort_by(|a, b| {
        let category_order = a.0.cmp(&b.0);
        // --------------------------------------------------
        // if categories are equal, compare project names
        // --------------------------------------------------
        match category_order {
            std::cmp::Ordering::Equal => a.1.0.cmp(&b.1.0),
            _ => category_order
        }
    });
    // --------------------------------------------------
    // return
    // --------------------------------------------------
    pages
});
pub static ALL_PROJECTS_HM: LazyLock<HashMap<String, Vec<ProjectTemplate>>> = LazyLock::new(|| ALL_PROJECTS
    .iter()
    .fold(HashMap::new(), |mut hm, (_, (category, project))| {
        match hm.contains_key(category) {
            true => hm.get_mut(category).unwrap().push(project.clone()),
            false => { let _ = hm.insert(category.clone(), vec![project.clone()]); },
        };
        hm
    })
);

#[derive(Debug, Clone, Template)]
#[template(path = "projects/project-template.html", print = "all")]
pub struct ProjectTemplate {
    pub name: String,
    pub url: String,
    pub content: String,
    pub header: ProjectHeader,
    pub sidebar: SidebarType,
}

#[derive(Debug, Clone, Deserialize)]
/// A JSON file describing a project, used at the top of 
/// every project page
pub struct ProjectHeader {
    pub title: String,
    #[serde(rename = "collab-type")]
    pub collab_type: String,
    pub status: String,
    #[serde(deserialize_with = "empty_as_none")]
    pub desc: Option<String>,
    #[serde(deserialize_with = "empty_as_none")]
    pub date: Option<String>,
    #[serde(deserialize_with = "empty_as_none")]
    pub location: Option<String>,
    #[serde(default)]
    pub repos: Vec<String>,
}

/// Deserializes an empty string into [`None`]
fn empty_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.filter(|s| !s.is_empty()))
}