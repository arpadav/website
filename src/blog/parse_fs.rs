// --------------------------------------------------
// local
// --------------------------------------------------
use super::BlogDateFormat;
use crate::prelude::*;

// --------------------------------------------------
// external
// --------------------------------------------------
use std::path::PathBuf;

// --------------------------------------------------
// statics
// --------------------------------------------------
/// Locally scoped blog pages, used commonly between [`BLOG_POSTS_META`] and [`BLOG_POSTS`]
///
/// Scans `content/blog/` for blog posts, extracts metadata,
/// sorts newest-first. Handles both flat `.md` files and
/// folders with `index.md`
pub(crate) static _INNER_BLOG_PAGES: LazyLock<Vec<ParsedBlogPost>> = LazyLock::new(|| {
    std::fs::read_dir(crate::BLOG_DIR)
        .expect("Failed to read blog directory")
        .filter_map(Result::ok)
        .filter_map(ParsedBlogPost::new)
        .collect()
});

/// A common struct for parsing blog posts from the filesystem
///
/// Used by both [`BlogPost`] and [`BlogPostTemplate`]
pub(crate) struct ParsedBlogPost {
    /// Page title
    pub(crate) title: String,
    /// File name, including extension
    pub(crate) filename: String,
    /// File stem, without extension
    pub(crate) filestem: String,
    /// Source file - will always be markdown
    pub(crate) src: PathBuf,
    /// parsed date from the filename
    pub(crate) date: BlogDateFormat,
}
/// [`ParsedBlogPost`] implementation
impl ParsedBlogPost {
    fn new(entry: std::fs::DirEntry) -> Option<ParsedBlogPost> {
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
                eprintln!("Blog folder `{filename}` missing index.md");
                return None;
            }
            (index_md, filename.clone())
        } else if filename.ends_with(".md") {
            let stem = filename.trim_end_matches(".md").to_string();
            (path.clone(), stem)
        } else {
            eprintln!("Found {} in the blog dir, not markdown, skipping", filename);
            return None;
        };
        // --------------------------------------------------
        // parse date
        // --------------------------------------------------
        let date: BlogDateFormat = filestem.parse().ok()?;
        // --------------------------------------------------
        // read markdown content once
        // --------------------------------------------------
        let md_content = std::fs::read_to_string(&src).ok()?;
        // --------------------------------------------------
        // extract title from first H1. if fails, take the file
        // stem, replace - with space, and capitalize
        // --------------------------------------------------
        let title = MarkdownDocument::extract_h1(&md_content).unwrap_or_else(|| {
            filestem[BlogDateFormat::PREFIX_LEN + 1..]
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
        Some(ParsedBlogPost {
            title,
            filename,
            filestem,
            src,
            date,
        })
    }
}
