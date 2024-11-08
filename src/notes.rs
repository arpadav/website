// --------------------------------------------------
// local
// --------------------------------------------------
use crate::prelude::*;

// --------------------------------------------------
// statics
// --------------------------------------------------
pub static NOTES: LazyLock<Vec<(String, Vec<(String, Vec<Link>)>)>> = LazyLock::new(|| {
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
                                let path = entry.path();
                                path.is_file().then(|| Link {
                                    name: note_name.clone(),
                                    url: format!("{}/{}/{}/{}", crate::url_relative_static!(crate::NOTES_DIR), category, topic, note_name),
                                })
                            })
                            .collect::<Vec<_>>();
                        // ----------------------------------------------------
                        // <<STYLE+TAG>>
                        // ----------------------------------------------------
                        sorted_notes.sort_by(|a, b| a.name.cmp(&b.name));
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
            notes: (*NOTES).clone(),
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