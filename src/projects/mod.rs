// --------------------------------------------------
// mods
// --------------------------------------------------
mod page;

// --------------------------------------------------
// local
// --------------------------------------------------
use crate::{deployutil::DEPLOYMENT_MAP, prelude::*};
use page::{ProjectHeader, ProjectTemplate};

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
///   file. If either file is missing, a warning is printed and the
///   project is then skipped.
pub static ALL_PROJECTS: LazyLock<Vec<(ProjectCategory, Vec<Page<ProjectTemplate>>)>> =
    LazyLock::new(|| {
        // --------------------------------------------------
        // loop through project categories
        // --------------------------------------------------
        let mut pages: Vec<(ProjectCategory, Vec<Page<ProjectTemplate>>)> = std::fs::read_dir(crate::PROJECT_CATEGORIES_DIR)
        .expect("Failed to read project categories directory")
        .filter_map(Result::ok)
        // --------------------------------------------------
        // get the category name, pass it down
        // --------------------------------------------------
        .filter_map(|category_entry| {
            let category_path = category_entry.path();
            let category_name = category_path.file_name()?.to_string_lossy().to_string();
            let category_projects = std::fs::read_dir(category_path.clone()).ok()?;
            category_path.is_dir().then_some((category_name, category_projects))
        })
        // --------------------------------------------------
        // handle invalid items
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
            let md_path = project_path.join(format!("{}.md", project_name));
            // --------------------------------------------------
            // get the year (get nums until alphabet hit)
            // --------------------------------------------------
            let project_start_year = project_name
                .chars()
                .take_while(|c| c.is_numeric())
                .collect::<String>();
            // --------------------------------------------------
            // if doesnt exist, print warning
            // --------------------------------------------------
            if !json_path.exists() {
                eprintln!("Warning: Missing JSON file for project: {}", project_path.display());
                return None;
            }
            let json_path = json_path.display().to_string();
            // --------------------------------------------------
            // parse json: if hidden, remove the entire project
            // folder subtree from deployment and return None
            // --------------------------------------------------
            let mut project_header: ProjectHeader = crate::json_template!(json_path);
            if project_header.hidden {
                // --------------------------------------------------
                // remove every entry under the project folder: the
                // catch-all `<file>` rule would otherwise still deploy
                // the project's assets (images, pdfs, gifs), leaving
                // them reachable even with no link or page
                // --------------------------------------------------
                DEPLOYMENT_MAP.w().remove_under(&project_path);
                return None;
            }
            // --------------------------------------------------
            // prepend the start year to the title
            // --------------------------------------------------
            let page_title = project_header.title.clone();
            project_header.title = format!("{project_start_year} - {}", project_header.title);
            // --------------------------------------------------
            // if doesnt exist, print warning
            // --------------------------------------------------
            // however, if both exist, panic! dont know which
            // one to use
            // --------------------------------------------------
            let (content, src_path, srctype) = match (html_path.exists(), md_path.exists()) {
                (true, true) => panic!("Error: Both HTML and Markdown files exist for project: {}. Only one can be used. Not deploying LOL!", project_path.display()),
                (false, false) => {
                    eprintln!("Warning: Missing HTML or Markdown file for project: {}", project_path.display());
                    return None;
                },
                (false, true) => (MarkdownDocument::from_file(&md_path, &project_name).html, md_path, SourceType::Markdown),
                (true, false) => (std::fs::read_to_string(&html_path).ok()?, html_path, SourceType::Html),
            };
            // --------------------------------------------------
            // return
            // --------------------------------------------------
            Some((
                category_name,
                src_path,
                ProjectTemplate {
                    title: crate::title!(page_title),
                    name: project_name.to_string(),
                    // --------------------------------------------------
                    // <<STYLE+TAG>>
                    // --------------------------------------------------
                    url: format!("/projects/{}/", project_name),
                    header: project_header,
                    content,
                    sidebar: SidebarType::Projects,
                    srctype,
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
            v.push((ProjectCategory::new(category.clone()), projects));
            v
        });
        // --------------------------------------------------
        // sort projects based off category (alphabetic, where projects
        // are labeled using `<num> <name>`)
        // then name (reverse alphabetic, which is actually reverse chronological
        // so that the most recent projects are first)
        // --------------------------------------------------
        // <<STYLE+TAG>>
        // --------------------------------------------------
        pages.sort_by(|a, b| a.0.raw_name.cmp(&b.0.raw_name));
        pages.iter_mut().for_each(|(_, categorized_projects)| {
            categorized_projects.sort_by(|a, b| b.page.name.cmp(&a.page.name))
        });
        // --------------------------------------------------
        // return
        // --------------------------------------------------
        pages
    });

#[derive(Debug, Clone)]
/// A project category with a name and a generated id
pub struct ProjectCategory {
    /// The raw name of the project category, used for generating the id
    ///
    /// E.g. 1. rust crates, 3. personal, etc. This is used to sort
    pub raw_name: String,
    /// The name / display name of the project category
    ///
    /// E.g. rust crates, personal, academic, professional
    pub name: String,
    /// The id of the project category, used for generating URLs
    pub id: String,
}
/// [`Category`] implementation
impl ProjectCategory {
    /// Creates a new [`Category`] with the given name and a generated id
    pub fn new(raw_name: String) -> Self {
        let name = raw_name
            .split_once(' ')
            .map(|(_, r)| r)
            .unwrap_or(raw_name.as_str())
            .to_owned();
        let id = name.to_lowercase().replace(' ', "-");
        Self { raw_name, name, id }
    }
}

#[derive(Debug, Clone, Template)]
#[template(path = "projects/projects-homepage.html")]
/// Homepage which shows all projects
pub struct ProjectsHomepage {
    pub title: String,
    pub sidebar: SidebarType,
}
/// [`ProjectsHomepage`] implementation of [`Create`]
impl Create for ProjectsHomepage {
    fn create() -> Self {
        Self {
            title: crate::title!("Projects"),
            sidebar: SidebarType::GatorOnly,
        }
    }
}
/// [`ProjectsHomepage`] implementation of [`SourcePath`]
impl SourcePath<ProjectsHomepage> for ProjectsHomepage {
    fn src_path() -> std::path::PathBuf {
        [crate::TEMPLATES_DIR, "/projects/projects-homepage.html"]
            .concat()
            .into()
    }
}
