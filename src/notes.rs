// --------------------------------------------------
// local
// --------------------------------------------------
use crate::prelude::*;

// --------------------------------------------------
// statics
// --------------------------------------------------
/// Iterates through notes directory, depth of 2. All
/// files auto-become links, and all folders auto-become
/// links at this depth. Folders must have an `index.md`
/// or an `index.html` file in order for the link to 
/// actually work 
pub static NOTES_LINKS_RAW: LazyLock<Vec<(String, Vec<(String, Vec<Link>)>)>> = LazyLock::new(|| {
    // --------------------------------------------------
    // read two deep. one for each category, one for each note topic
    // use fs dir
    // --------------------------------------------------
    std::fs::read_dir(crate::NOTES_DIR)
    .expect("Failed to read notes directory for categories (e.g. academic, personal)")
    .into_iter()
    .filter_map(Result::ok)
    .filter_map(|entry| {
        let category = entry.file_name().into_string().ok()?;
        let path = entry.path();
        path.is_dir().then(|| {
            let mut sorted_topics =  std::fs::read_dir(&path)
                .expect("Failed to read sub-notes directory for topics (e.g. class names, math, misc, thoughts, etc.)")
                .into_iter()
                .filter_map(Result::ok)
                .filter_map(|entry| {
                    let topic = entry.file_name().into_string().ok()?;
                    let path = entry.path();
                    path.is_dir().then(|| {
                        let mut sorted_notes = std::fs::read_dir(&path)
                            .expect("Failed to read sub-notes directory for notes (e.g. actual note files, .pdfs, .md, etc.)")
                            .into_iter()
                            .filter_map(Result::ok)
                            .filter_map(|entry| {
                                let note_name = entry.file_name().into_string().ok()?;
                                Some(Link {
                                    name: note_name.clone(),
                                    url: format!("{}/{}/{}/{}", crate::NOTES_DIR, category, topic, note_name),
                                })
                            })
                            .collect::<Vec<_>>();
                        // ----------------------------------------------------
                        // <<STYLE+TAG>>
                        // ----------------------------------------------------
                        sorted_notes.sort_by(|a, b| b.name.cmp(&a.name));
                        (
                            topic.clone(),
                            sorted_notes,
                        )
                    })
                })
                .collect::<Vec<_>>();
            // ----------------------------------------------------
            // <<STYLE+TAG>>
            // ----------------------------------------------------
            sorted_topics.sort_by(|a, b| b.0.cmp(&a.0));
            (
                category.clone(),
                sorted_topics,
            )
        })
    })
    .collect::<Vec<_>>()
});
/// Markdown -> HTML, or HTML contents to be displayed
/// on a notes page
pub static NOTES: LazyLock<Vec<Page<NotesTemplate>>> = LazyLock::new(|| {
    NOTES_LINKS_RAW
        .iter()
        .map(|(_, x)| {
            x.iter().map(|(_, x)| {
                x.iter()
                    .filter(|x| std::path::Path::new(&x.url).extension().is_none())
                    .map(|x| x.clone())
                    .map(|x| {
                        let index_html = std::path::Path::new(&x.url).join("index.html");
                        let index_md = std::path::Path::new(&x.url).join("index.md");
                        let (src, content) = match index_html.exists() {
                            true => (
                                index_html.clone(),
                                std::fs::read_to_string(&index_html).expect(format!("Failed to open index.html for note `{}`", x.name).as_str())
                            ),
                            false => match index_md.exists() {
                                true => (
                                    index_md.clone(),
                                    String::from_utf8_lossy(&std::process::Command::new("pandoc")
                                        .arg(&index_md)
                                        .arg("--to")
                                        .arg("html")
                                        .arg("-s")
                                        // ----------------------------------------------------
                                        // <<STYLE+TAG>>
                                        // ----------------------------------------------------
                                        // .arg("--css")
                                        // .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/static/css/std.css"))
                                        .arg("--highlight-style=zenburn")
                                        .output()
                                        .expect(format!("Failed to run `pandoc` for note `{}`", x.name).as_str())
                                        .stdout
                                    ).to_string()),
                                false => panic!("Failed to find index.html or index.md for note `{}`", x.name),
                            }       
                        };
                        let content = content.replace("’", "'");
                        let content = content.replace("“", "\"");
                        let content = content.replace("”", "\"");
                        Page {
                            src,
                            page: NotesTemplate {
                                content,
                                ..Default::default()
                            }
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>()
        })
        .flatten()
        .collect()  
});
/// Links to notes on the deployment end. Just a massive filter
pub static NOTES_LINKS: LazyLock<Vec<(String, Vec<(String, Vec<Link>)>)>> = LazyLock::new(|| {
    NOTES_LINKS_RAW.iter().map(|(category, topics)| {(
        category.clone(),
        topics.iter().map(|(topic, notes)| {(
            topic.clone(),
            notes.iter().map(|x| Link {
                name: x.name.clone(),
                url: crate::url_relative_content!(&x.url),
            })
            .collect::<Vec<_>>(),
        )})
        .collect::<Vec<_>>(),
    )})
    .collect::<Vec<_>>()
});

#[derive(Template, Default)]
#[template(path = "notes.html")]
/// Template for homepage of notes
pub struct NotesHomepage {
    pub sidebar: SidebarType,
    pub notes: Vec<(String, Vec<(String, Vec<Link>)>)>,
}
/// [`NotesHomepage`] implmentation of [`Create`]
impl Create for NotesHomepage {
    fn create() -> Self {
        Self {
            notes: (*NOTES_LINKS).clone(),
            ..Default::default()
        }
    }
}
/// [`NotesHomepage`] implmentation of [`SourcePath`]
impl SourcePath<NotesHomepage> for NotesHomepage {
    fn src_path() -> std::path::PathBuf {
        [ crate::TEMPLATES_DIR, "/notes.html" ].concat().into()
    }
}

#[derive(Template, Default)]
#[template(path = "general/markdown.html")]
/// Template for notes pages
pub struct NotesTemplate {
    pub sidebar: SidebarType,
    pub content: String,
}