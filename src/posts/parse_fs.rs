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
/// Scans `content/posts/` for posts, extracts metadata,
/// sorts newest-first. Handles both flat `.md` files and
/// folders with `index.md`
pub(crate) static _INNER_POSTS_PAGES: LazyLock<Vec<ParsedPost>> = LazyLock::new(|| {
    std::fs::read_dir(crate::POSTS_DIR)
        .expect("Failed to read posts directory")
        .filter_map(Result::ok)
        .filter_map(ParsedPost::new)
        .collect()
});

/// A common struct for parsing posts from the filesystem
///
/// Used by both [`Post`] and [`PostTemplate`]
pub(crate) struct ParsedPost {
    /// Page title
    pub(crate) title: String,
    /// File name, including extension
    pub(crate) filename: String,
    /// File stem, without extension
    pub(crate) filestem: String,
    /// Source file - will always be markdown
    pub(crate) src: PathBuf,
    /// parsed date from the filename
    pub(crate) date: PostDateFormat,
}
/// [`ParsedPost`] implementation
impl ParsedPost {
    fn new(entry: std::fs::DirEntry) -> Option<ParsedPost> {
        // --------------------------------------------------
        // get the filename and its path
        // --------------------------------------------------
        let filename = entry.file_name().into_string().ok()?;
        let path = entry.path();
        // --------------------------------------------------
        // determine source file
        // --------------------------------------------------
        let (src, filestem) = if path.is_dir() {
            let index_md = path.join("index.md");
            if !index_md.exists() {
                eprintln!("Post folder `{filename}` missing index.md");
                return None;
            }
            (index_md, filename.clone())
        } else if filename.ends_with(".md") {
            let stem = filename.trim_end_matches(".md").to_string();
            (path.clone(), stem)
        } else {
            eprintln!("Found {} in the posts dir, not markdown, skipping", filename);
            return None;
        };
        // --------------------------------------------------
        // parse date
        // --------------------------------------------------
        let date: PostDateFormat = filestem.parse().ok()?;
        // --------------------------------------------------
        // read markdown content once
        // --------------------------------------------------
        let md_content = std::fs::read_to_string(&src).ok()?;
        // --------------------------------------------------
        // extract title from first H1. if fails, take the file
        // stem, replace - with space, and capitalize
        // --------------------------------------------------
        let title = MarkdownDocument::extract_h1(&md_content).unwrap_or_else(|| {
            filestem[PostDateFormat::PREFIX_LEN + 1..]
                .replace('-', " ")
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
            filename,
            filestem,
            src,
            date,
        })
    }
}
