// --------------------------------------------------
// mods
// --------------------------------------------------
mod blog_date;

// --------------------------------------------------
// local
// --------------------------------------------------
use crate::prelude::*;
use blog_date::BlogDateFormat;

// --------------------------------------------------
// statics
// --------------------------------------------------
/// Scans `content/blog/` for blog posts, extracts metadata,
/// sorts newest-first. Handles both flat `.md` files and
/// folders with `index.md`.
pub static BLOG_POSTS_META: LazyLock<Vec<BlogPost>> = LazyLock::new(|| {
    let mut posts: Vec<BlogPost> = std::fs::read_dir(crate::BLOG_DIR)
        .expect("Failed to read blog directory")
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let name = entry.file_name().into_string().ok()?;
            let path = entry.path();
            // --------------------------------------------------
            // determine the markdown source file
            // --------------------------------------------------
            let md_path = if path.is_dir() {
                let index_md = path.join("index.md");
                if !index_md.exists() {
                    panic!("Blog folder `{}` missing index.md", name);
                }
                index_md
            } else if name.ends_with(".md") {
                path.clone()
            } else {
                return None;
            };
            // --------------------------------------------------
            // parse date from filename/folder name
            // --------------------------------------------------
            let stem = if path.is_dir() {
                name.clone()
            } else {
                name.trim_end_matches(".md").to_string()
            };
            let date: BlogDateFormat = stem.parse().unwrap();
            // --------------------------------------------------
            // extract title from first H1
            // --------------------------------------------------
            let md_content = std::fs::read_to_string(&md_path)
                .unwrap_or_else(|_| panic!("Failed to read blog post `{}`", name));
            let title = MarkdownDocument::extract_h1(&md_content).unwrap_or_else(|| {
                stem[BlogDateFormat::PREFIX_LEN + 1..]
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
            // build url
            // --------------------------------------------------
            let url = format!("/blog/{}", stem);
            Some(BlogPost {
                title,
                date_raw: date.sort_key(),
                date: date.to_string(),
                url,
            })
        })
        .collect();
    // --------------------------------------------------
    // newest first
    // --------------------------------------------------
    posts.sort_by(|a, b| b.date_raw.cmp(&a.date_raw));
    posts
});

/// Blog post pages: markdown -> HTML for each post
pub static BLOG_PAGES: LazyLock<Vec<Page<BlogPostTemplate>>> = LazyLock::new(|| {
    std::fs::read_dir(crate::BLOG_DIR)
        .expect("Failed to read blog directory")
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let name = entry.file_name().into_string().ok()?;
            let path = entry.path();
            // --------------------------------------------------
            // determine source file
            // --------------------------------------------------
            let (md_path, src) = if path.is_dir() {
                let index_md = path.join("index.md");
                if !index_md.exists() {
                    return None;
                }
                (index_md.clone(), index_md)
            } else if name.ends_with(".md") {
                (path.clone(), path.clone())
            } else {
                return None;
            };
            // --------------------------------------------------
            // parse metadata
            // --------------------------------------------------
            let stem = if path.is_dir() {
                name.clone()
            } else {
                name.trim_end_matches(".md").to_string()
            };
            let date: BlogDateFormat = stem.parse().unwrap_or_else(|e| panic!("{}", e));
            let md_content = std::fs::read_to_string(&md_path)
                .unwrap_or_else(|_| panic!("Failed to read blog post `{}`", name));
            let post_title = MarkdownDocument::extract_h1(&md_content)
                .unwrap_or_else(|| stem[BlogDateFormat::PREFIX_LEN + 1..].replace('-', " "));
            // --------------------------------------------------
            // convert to HTML + strip first H1
            // --------------------------------------------------
            let doc = MarkdownDocument::from_file(&src, &name);
            let content = MarkdownDocument::strip_first_h1(&doc.html);
            let toc = MarkdownDocument::extract_toc(&content);
            let content = MarkdownDocument::inject_anchor_links(&content); // overwrite content with anchor links
            Some(Page {
                src,
                page: BlogPostTemplate {
                    title: crate::title!(post_title),
                    sidebar: SidebarType::Blog,
                    post_title: post_title.clone(),
                    date: date.to_string(),
                    content,
                    toc,
                },
            })
        })
        .collect()
});

#[derive(Template, Default)]
#[template(path = "blog/blog.html")]
/// Template for blog homepage / listing page
pub struct BlogHomepage {
    title: String,
    pub sidebar: SidebarType,
    pub posts: Vec<BlogPost>,
}
/// [`BlogHomepage`] implementation of [`Create`]
impl Create for BlogHomepage {
    fn create() -> Self {
        Self {
            title: crate::title!("Blog"),
            sidebar: SidebarType::GatorOnly,
            posts: (*BLOG_POSTS_META).clone(),
        }
    }
}
/// [`BlogHomepage`] implementation of [`SourcePath`]
impl SourcePath<BlogHomepage> for BlogHomepage {
    fn src_path() -> std::path::PathBuf {
        [crate::TEMPLATES_DIR, "/blog/blog.html"].concat().into()
    }
}

#[derive(Template, Default)]
#[template(path = "blog/post.html")]
/// Template for individual blog post pages
pub struct BlogPostTemplate {
    title: String,
    pub sidebar: SidebarType,
    pub post_title: String,
    pub date: String,
    pub content: String,
    pub toc: Vec<TocEntry>,
}

#[derive(Clone, Debug)]
/// Metadata for a single blog post
pub struct BlogPost {
    /// Display title (from first H1 in markdown)
    pub title: String,
    /// Formatted display date, e.g. "March 29, 2026"
    pub date: String,
    /// Raw YYYYMMDDHHMM string for sorting
    pub date_raw: String,
    /// Deployment URL, e.g. "/blog/20260329-hello-world"
    pub url: String,
}
