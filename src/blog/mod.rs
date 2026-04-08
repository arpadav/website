// --------------------------------------------------
// mods
// --------------------------------------------------
mod blog_date;
mod parse_fs;

// --------------------------------------------------
// local
// --------------------------------------------------
use crate::prelude::*;
use blog_date::BlogDateFormat;
use parse_fs::_INNER_BLOG_PAGES;

// --------------------------------------------------
// statics
// --------------------------------------------------
/// Metadata for blog posts, used for indexing and display
pub static BLOG_POSTS_META: LazyLock<Vec<BlogPost>> = LazyLock::new(|| {
    LazyLock::force(&_INNER_BLOG_PAGES);
    let mut posts: Vec<BlogPost> = _INNER_BLOG_PAGES
        .iter()
        .map(|p| BlogPost {
            title: p.title.clone(),
            date_raw: p.date.as_key(),
            date: p.date.to_string(),
            url: format!("/blog/{}", p.filestem),
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
    LazyLock::force(&_INNER_BLOG_PAGES);
    _INNER_BLOG_PAGES
        .iter()
        .map(|p| {
            // --------------------------------------------------
            // convert to HTML + strip first H1
            // --------------------------------------------------
            let doc = MarkdownDocument::from_file(&p.src, &p.filename);
            let content = MarkdownDocument::strip_first_h1(&doc.html);
            let toc = MarkdownDocument::extract_toc(&content);
            let content = MarkdownDocument::inject_anchor_links(&content); // overwrite content with anchor links
            Page {
                src: p.src.clone(),
                page: BlogPostTemplate {
                    title: crate::title!(p.title),
                    post_title: p.title.clone(),
                    sidebar: SidebarType::Blog,
                    date: p.date.to_string(),
                    content,
                    toc,
                },
            }
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
///
/// This renders the markdown as HTML
pub struct BlogPostTemplate {
    /// Page title, e.g. "Hello World"
    title: String,
    /// Post title, see [`BlogPost::title`]
    pub post_title: String,
    /// Sidebar type, e.g. [`SidebarType::Blog`]
    pub sidebar: SidebarType,
    /// Formatted display date, see [`BlogPost::date`]
    pub date: String,
    /// The rendered HTML content of the post
    pub content: String,
    /// Table of contents entries, see [`TocEntry`]
    pub toc: Vec<TocEntry>,
}

#[derive(Clone, Debug)]
/// Metadata for a single blog post
///
/// This is lightweight and mainly used for indexing
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
