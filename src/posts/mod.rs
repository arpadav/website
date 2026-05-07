// --------------------------------------------------
// mods
// --------------------------------------------------
mod parse_fs;
mod posts_date;

// --------------------------------------------------
// local
// --------------------------------------------------
use crate::prelude::*;
use parse_fs::_INNER_POSTS_PAGES;
use posts_date::PostDateFormat;

// --------------------------------------------------
// statics
// --------------------------------------------------
/// Metadata for posts, used for indexing and display
pub static POSTS_META: LazyLock<Vec<Post>> = LazyLock::new(|| {
    LazyLock::force(&_INNER_POSTS_PAGES);
    // --------------------------------------------------
    // same-day same-slug collision check (URLs would clash)
    // --------------------------------------------------
    _INNER_POSTS_PAGES.iter().fold(
        std::collections::HashMap::new(),
        |mut seen: std::collections::HashMap<String, std::path::PathBuf>, p| {
            let key = format!(
                "{:04}/{:02}/{:02}/{}",
                p.date.year, p.date.month, p.date.day, p.slug
            );
            if let Some(prev) = seen.insert(key.clone(), p.src.clone()) {
                panic!(
                    "Same-day URL collision /posts/{}: `{}` and `{}`",
                    key,
                    prev.display(),
                    p.src.display()
                );
            }
            seen
        },
    );
    // --------------------------------------------------
    // get posts
    // --------------------------------------------------
    let mut posts: Vec<Post> = _INNER_POSTS_PAGES
        .iter()
        .map(|p| Post {
            title: p.title.clone(),
            date_raw: p.date.as_key(),
            date: p.date.to_string(),
            url: format!(
                "/posts/{:04}/{:02}/{:02}/{}",
                p.date.year, p.date.month, p.date.day, p.slug
            ),
            slug: p.slug.clone(),
        })
        .collect();
    // --------------------------------------------------
    // newest first; tiebreak alphabetical by slug
    // --------------------------------------------------
    posts.sort_by(|a, b| {
        b.date_raw
            .cmp(&a.date_raw)
            .then_with(|| a.slug.cmp(&b.slug))
    });
    posts
});

/// Post pages: markdown -> HTML for each post
pub static POSTS_PAGES: LazyLock<Vec<Page<PostTemplate>>> = LazyLock::new(|| {
    LazyLock::force(&_INNER_POSTS_PAGES);
    _INNER_POSTS_PAGES
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
                page: PostTemplate {
                    title: crate::title!(p.title),
                    post_title: p.title.clone(),
                    sidebar: SidebarType::Posts,
                    date: p.date.to_string(),
                    content,
                    toc,
                },
            }
        })
        .collect()
});

#[derive(Template, Default)]
#[template(path = "posts/posts.html")]
/// Template for posts homepage / listing page
pub struct PostsHomepage {
    title: String,
    pub sidebar: SidebarType,
    pub posts: Vec<Post>,
}
/// [`PostsHomepage`] implementation of [`Create`]
impl Create for PostsHomepage {
    fn create() -> Self {
        Self {
            title: crate::title!("Posts"),
            sidebar: SidebarType::GatorOnly,
            posts: (*POSTS_META).clone(),
        }
    }
}
/// [`PostsHomepage`] implementation of [`SourcePath`]
impl SourcePath<PostsHomepage> for PostsHomepage {
    fn src_path() -> std::path::PathBuf {
        [crate::TEMPLATES_DIR, "/posts/posts.html"].concat().into()
    }
}

#[derive(Template, Default)]
#[template(path = "posts/post.html")]
/// Template for individual post pages
///
/// This renders the markdown as HTML
pub struct PostTemplate {
    /// Page title, e.g. "Hello World"
    title: String,
    /// Post title, see [`Post::title`]
    pub post_title: String,
    /// Sidebar type, e.g. [`SidebarType::Posts`]
    pub sidebar: SidebarType,
    /// Formatted display date, see [`Post::date`]
    pub date: String,
    /// The rendered HTML content of the post
    pub content: String,
    /// Table of contents entries, see [`TocEntry`]
    pub toc: Vec<TocEntry>,
}

#[derive(Clone, Debug)]
/// Metadata for a single post
///
/// This is lightweight and mainly used for indexing
pub struct Post {
    /// Display title (from first H1 in markdown)
    pub title: String,
    /// Formatted display date, e.g. "March 29, 2026"
    pub date: String,
    /// Raw YYYYMMDD string for sorting (date desc, slug asc tiebreak)
    pub date_raw: String,
    /// Deployment URL, e.g. "/posts/2026/03/29/hello-world"
    pub url: String,
    /// Slug (leaf dir name), used as sort tiebreak for same-day posts
    pub slug: String,
}
