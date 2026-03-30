// --------------------------------------------------
// local
// --------------------------------------------------
use crate::prelude::*;

// --------------------------------------------------
// types
// --------------------------------------------------
type Notes = Vec<(String, Vec<(String, Vec<Link>)>)>;

// --------------------------------------------------
// statics
// --------------------------------------------------
/// Iterates through notes directory, depth of 2. All
/// files auto-become links, and all folders auto-become
/// links at this depth. Folders must have an `index.md`
/// or an `index.html` file in order for the link to
/// actually work
pub static NOTES_LINKS_RAW: LazyLock<Notes> = LazyLock::new(|| {
    // --------------------------------------------------
    // read two deep. one for each category, one for each note topic
    // use fs dir
    // --------------------------------------------------
    let mut sorted_topics = std::fs::read_dir(crate::NOTES_DIR)
        .expect("Failed to read notes directory for categories (e.g. academic, personal)")
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let category = entry.file_name().into_string().ok()?;
            let path = entry.path();
            path.is_dir().then(|| {
                let mut sorted_topics =  std::fs::read_dir(&path)
                    .expect("Failed to read sub-notes directory for topics (e.g. class names, math, misc, thoughts, etc.)")
                    .filter_map(Result::ok)
                    .filter_map(|entry| {
                        let topic = entry.file_name().into_string().ok()?;
                        let path = entry.path();
                        path.is_dir().then(|| {
                            let mut sorted_notes = std::fs::read_dir(&path)
                                .expect("Failed to read sub-notes directory for notes (e.g. actual note files, .pdfs, .md, etc.)")
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
        }
    )
    .collect::<Vec<_>>();
    // ----------------------------------------------------
    // <<STYLE+TAG>>
    // ----------------------------------------------------
    sorted_topics.sort_by(|a, b| b.0.cmp(&a.0));
    sorted_topics
});
/// Markdown -> HTML, or HTML contents to be displayed
/// on a notes page
pub static NOTES: LazyLock<Vec<Page<NotesTemplate>>> = LazyLock::new(|| {
    NOTES_LINKS_RAW
        .iter()
        .flat_map(|(_, x)| {
            x.iter().flat_map(|(_, x)| {
                x.iter()
                    .filter(|x| !x.url.ends_with(".pdf"))
                    .map(|x| {
                        let path = std::path::Path::new(&x.url);
                        let (src, source_type) = MarkdownDocument::resolve_source(path, &x.name);
                        let content = match source_type {
                            SourceType::Html => {
                                MarkdownDocument::from_html_file(&src, &x.name).html
                            }
                            SourceType::Markdown => MarkdownDocument::from_file(&src, &x.name).html,
                        };
                        Page {
                            src,
                            page: NotesTemplate {
                                title: crate::title!(x.name),
                                content,
                                ..Default::default()
                            },
                        }
                    })
                    .collect::<Vec<_>>()
            })
        })
        .collect()
});
/// Links to notes on the deployment end. Just a massive filter
pub static NOTES_LINKS: LazyLock<Notes> = LazyLock::new(|| {
    NOTES_LINKS_RAW
        .iter()
        .map(|(category, topics)| {
            (
                category.clone(),
                topics
                    .iter()
                    .map(|(topic, notes)| {
                        (
                            topic.clone(),
                            notes
                                .iter()
                                .map(|x| Link {
                                    name: x.name.clone(),
                                    url: crate::url_relative_content!(&x.url),
                                })
                                .collect::<Vec<_>>(),
                        )
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>()
});

#[derive(Template, Default)]
#[template(path = "notes.html")]
/// Template for homepage of notes
pub struct NotesHomepage {
    title: String,
    pub sidebar: SidebarType,
    pub notes: Notes,
}
/// [`NotesHomepage`] implmentation of [`Create`]
impl Create for NotesHomepage {
    fn create() -> Self {
        Self {
            title: crate::title!("Notes"),
            notes: (*NOTES_LINKS).clone(),
            ..Default::default()
        }
    }
}
/// [`NotesHomepage`] implmentation of [`SourcePath`]
impl SourcePath<NotesHomepage> for NotesHomepage {
    fn src_path() -> std::path::PathBuf {
        [crate::TEMPLATES_DIR, "/notes.html"].concat().into()
    }
}

#[derive(Template, Default)]
#[template(path = "general/markdown.html")]
/// Template for notes pages
pub struct NotesTemplate {
    title: String,
    pub sidebar: SidebarType,
    pub content: String,
}
