// --------------------------------------------------
// local
// --------------------------------------------------
use super::PostDateFormat;
use crate::prelude::*;

// --------------------------------------------------
// external
// --------------------------------------------------
use std::path::PathBuf;

// --------------------------------------------------
// statics
// --------------------------------------------------
/// Locally scoped post pages, used commonly between [`POSTS_META`] and [`POSTS_PAGES`]
///
/// Walks `content/posts/YYYY/MM/DD/<slug>/index.md`, extracts metadata.
/// Sorting happens in `mod.rs`.
pub(crate) static _INNER_POSTS_PAGES: LazyLock<Vec<ParsedPost>> = LazyLock::new(|| {
    let posts_dir = std::path::Path::new(crate::POSTS_DIR);
    let mut out = Vec::new();
    for (year, year_path) in subdirs(posts_dir) {
        for (month, month_path) in subdirs(&year_path) {
            for (day, day_path) in subdirs(&month_path) {
                let date = match PostDateFormat::from_path_parts(&year, &month, &day) {
                    Ok(d) => d,
                    Err(e) => {
                        eprintln!("Skipping {}: {}", day_path.display(), e);
                        continue;
                    }
                };
                for entry in read_dir_ok(&day_path) {
                    if let Some(p) = ParsedPost::new(entry, date.clone()) {
                        out.push(p);
                    }
                }
            }
        }
    }
    out
});

/// A common struct for parsing posts from the filesystem
///
/// Used by both [`Post`] and [`PostTemplate`]
pub(crate) struct ParsedPost {
    /// Page title
    pub(crate) title: String,
    /// File name (the leaf slug dir name)
    pub(crate) filename: String,
    /// Slug, leaf directory name (used for URL)
    pub(crate) slug: String,
    /// Source file, will always be `<slug>/index.md` (unlike projects)
    pub(crate) src: PathBuf,
    /// Parsed date from the parent directory components
    pub(crate) date: PostDateFormat,
}
/// [`ParsedPost`] implementation
impl ParsedPost {
    fn new(entry: std::fs::DirEntry, date: PostDateFormat) -> Option<ParsedPost> {
        // --------------------------------------------------
        // each post is a `<slug>/` dir containing `index.md`
        // --------------------------------------------------
        let path = entry.path();
        if !path.is_dir() {
            eprintln!("Skipping non-dir post entry: {}", path.display());
            return None;
        }
        let slug = entry.file_name().into_string().ok()?;
        let src = path.join("index.md");
        if !src.exists() {
            eprintln!("Post folder `{}` missing index.md", path.display());
            return None;
        }
        // --------------------------------------------------
        // read markdown content once
        // --------------------------------------------------
        let md_content = std::fs::read_to_string(&src).ok()?;
        // --------------------------------------------------
        // extract title from first H1. if fails, derive from
        // slug: replace - with space, capitalize each word.
        // --------------------------------------------------
        let title = MarkdownDocument::extract_h1(&md_content).unwrap_or_else(|| {
            slug.replace('-', " ")
                .split_whitespace()
                .map(|w| {
                    let mut c = w.chars();
                    match c.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().to_string() + c.as_str(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" ")
        });
        // --------------------------------------------------
        // return
        // --------------------------------------------------
        Some(ParsedPost {
            title,
            filename: slug.clone(),
            slug,
            src,
            date,
        })
    }
}

/// Helper function to read a dir, returning entries if okay
fn read_dir_ok(path: &std::path::Path) -> Vec<std::fs::DirEntry> {
    let Ok(entries) = std::fs::read_dir(path).map_err(|e| {
        eprintln!("Failed to read {}: {}", path.display(), e);
    }) else {
        return vec![];
    };
    entries.filter_map(Result::ok).collect()
}

/// Helper function to read subdirectories of a given path
fn subdirs(path: &std::path::Path) -> Vec<(String, std::path::PathBuf)> {
    read_dir_ok(path)
        .into_iter()
        .filter_map(|entry| {
            if !entry.path().is_dir() {
                return None;
            }
            let name = entry.file_name().into_string().ok()?;
            Some((name, entry.path()))
        })
        .collect()
}
