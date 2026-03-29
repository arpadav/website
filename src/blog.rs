// --------------------------------------------------
// local
// --------------------------------------------------
use crate::prelude::*;

// --------------------------------------------------
// constants
// --------------------------------------------------
const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

// --------------------------------------------------
// types
// --------------------------------------------------
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

/// Parse a YYYYMMDD-HHMM prefix from a filename into (raw, display) date strings.
/// Returns raw as "YYYYMMDDHHMM" (12 digits) for sorting.
/// Display stays human-readable, e.g. "March 29, 2026".
fn parse_blog_date(filename: &str) -> (String, String) {
    assert!(
        filename.len() >= 13,
        "Blog filename `{}` too short - expected YYYYMMDD-HHMM prefix",
        filename
    );
    let date_part = &filename[..8];
    let time_part = &filename[9..13];
    assert!(
        filename.as_bytes()[8] == b'-',
        "Blog filename `{}` missing hyphen after YYYYMMDD",
        filename
    );
    let year: u32 = date_part[..4]
        .parse()
        .unwrap_or_else(|_| panic!("Invalid year in blog filename `{}`", filename));
    let month: u32 = date_part[4..6]
        .parse()
        .unwrap_or_else(|_| panic!("Invalid month in blog filename `{}`", filename));
    let day: u32 = date_part[6..8]
        .parse()
        .unwrap_or_else(|_| panic!("Invalid day in blog filename `{}`", filename));
    let _hour: u32 = time_part[..2]
        .parse()
        .unwrap_or_else(|_| panic!("Invalid hour in blog filename `{}`", filename));
    let _minute: u32 = time_part[2..4]
        .parse()
        .unwrap_or_else(|_| panic!("Invalid minute in blog filename `{}`", filename));
    assert!(
        (1..=12).contains(&month),
        "Month out of range in blog filename `{}`",
        filename
    );
    assert!(
        (1..=31).contains(&day),
        "Day out of range in blog filename `{}`",
        filename
    );
    let raw = format!("{}{}", date_part, time_part);
    let display = format!("{} {}, {}", MONTHS[(month - 1) as usize], day, year);
    (raw, display)
}

/// Extract the first `# ...` heading from raw markdown content.
/// Returns None if no H1 is found.
fn extract_h1(md_content: &str) -> Option<String> {
    md_content
        .lines()
        .find(|line| line.starts_with("# "))
        .map(|line| line.trim_start_matches("# ").trim().to_string())
}

/// Strip the first `<h1>...</h1>` block from HTML content
/// so the template can render the title separately.
fn strip_first_h1(html: &str) -> String {
    if let Some(start) = html.find("<h1") {
        if let Some(end) = html[start..].find("</h1>") {
            let end_abs = start + end + "</h1>".len();
            return format!("{}{}", &html[..start], &html[end_abs..]);
        }
    }
    html.to_string()
}

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
            let (date_raw, date) = parse_blog_date(&stem);
            // --------------------------------------------------
            // extract title from first H1
            // --------------------------------------------------
            let md_content = std::fs::read_to_string(&md_path)
                .unwrap_or_else(|_| panic!("Failed to read blog post `{}`", name));
            let title = extract_h1(&md_content).unwrap_or_else(|| {
                // fallback: derive from slug
                stem[14..]
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
                date,
                date_raw,
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
            let (_, date) = parse_blog_date(&stem);
            let md_content = std::fs::read_to_string(&md_path)
                .unwrap_or_else(|_| panic!("Failed to read blog post `{}`", name));
            let post_title =
                extract_h1(&md_content).unwrap_or_else(|| stem[14..].replace('-', " "));
            // --------------------------------------------------
            // convert to HTML + strip first H1
            // --------------------------------------------------
            let content = md2html(&src, &name);
            let content = strip_first_h1(&content);
            let content = content.replace("\u{2018}", "'");
            let content = content.replace("\u{201c}", "\"");
            let content = content.replace("\u{201d}", "\"");

            Some(Page {
                src,
                page: BlogPostTemplate {
                    title: crate::title!(post_title),
                    sidebar: SidebarType::Blog,
                    post_title: post_title.clone(),
                    date,
                    content,
                },
            })
        })
        .collect()
});

// --------------------------------------------------
// templates
// --------------------------------------------------
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
}
