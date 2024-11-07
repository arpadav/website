// --------------------------------------------------
// local
// --------------------------------------------------
use crate::prelude::*;

#[derive(Template)]
#[template(path = "notes.html")]
/// Template for homepage of notes
pub struct NotesHomepage {
    pub bruh: std::marker::PhantomData<()>,
}
/// [`NotesHomepage`] implmentation of [`Create`]
impl Create for NotesHomepage {
    fn create() -> Self {
        Self {
            bruh: std::marker::PhantomData::default(),
            // tabs: (*tabs::ALL_TABS).clone(),
        }
    }
}
/// [`NotesHomepage`] implmentation of [`SourcePath`]
impl SourcePath<NotesHomepage> for NotesHomepage {
    fn src_path() -> std::path::PathBuf {
        [ crate::TEMPLATES_DIR, "/notes.html" ].concat().into()
    }
}