// --------------------------------------------------
// local
// --------------------------------------------------
use crate::primitives::SourceType;

// --------------------------------------------------
// external
// --------------------------------------------------
use std::path::{Path, PathBuf};

/// A markdown document converted to HTML with standard post-processing applied.
pub struct MarkdownDocument {
    /// The html content of the markdown document, with smart quotes normalized and the first `<h1>` stripped
    pub html: String,
}
/// [`MarkdownDocument`] implementation
impl MarkdownDocument {
    /// Convert a markdown file to HTML via pandoc, then normalize smart quotes.
    pub fn from_file(path: &Path, name: &str) -> Self {
        let html = Self::md2html(path, name);
        Self {
            html: Self::normalize_quotes(&html),
        }
    }

    /// Read an HTML file and normalize its smart quotes.
    pub fn from_html_file(path: &Path, name: &str) -> Self {
        let html = std::fs::read_to_string(path).unwrap_or_else(|_| {
            panic!(
                "Failed to open {}.html for `{}`",
                path.file_name().unwrap().to_string_lossy(),
                name
            )
        });
        Self {
            html: Self::normalize_quotes(&html),
        }
    }

    /// Resolve the content source file from a path that may be a file or a
    /// directory containing `index.html` / `index.md`.
    pub fn resolve_source(path: &Path, name: &str) -> (PathBuf, SourceType) {
        match path.extension() {
            // ----------------------------------------------------
            // if folder, find the index
            // ----------------------------------------------------
            // <<STYLE+TAG>>
            // ----------------------------------------------------
            None => {
                let index_html = path.join("index.html");
                let index_md = path.join("index.md");
                if index_html.exists() {
                    (index_html, SourceType::Html)
                } else if index_md.exists() {
                    (index_md, SourceType::Markdown)
                } else {
                    panic!("Failed to find index.html or index.md for `{}`", name);
                }
            }
            Some(ext) => {
                let source_type = match ext.to_str() {
                    Some("html") => SourceType::Html,
                    Some("md") => SourceType::Markdown,
                    _ => unimplemented!(
                        "Unsupported file type for {}, please implement how to convert it to HTML for display.\n\
                         If link is sufficient to view stand-alone (like a .pdf) please skip it in the filter call above.",
                        name
                    ),
                };
                (path.to_path_buf(), source_type)
            }
        }
    }

    /// Extract the first `# ...` heading from raw markdown content.
    pub fn extract_h1(md_content: &str) -> Option<String> {
        md_content
            .lines()
            .find(|line| line.starts_with("# "))
            .map(|line| line.trim_start_matches("# ").trim().to_string())
    }

    /// Strip the first `<h1>...</h1>` block from HTML so the template can
    /// render the title separately.
    pub fn strip_first_h1(html: &str) -> String {
        if let Some(start) = html.find("<h1") {
            if let Some(end) = html[start..].find("</h1>") {
                let end_abs = start + end + "</h1>".len();
                return format!("{}{}", &html[..start], &html[end_abs..]);
            }
        }
        html.to_string()
    }

    /// Replace Unicode smart quotes with their ASCII equivalents.
    fn normalize_quotes(html: &str) -> String {
        html.replace(['\u{2018}', '\u{2019}'], "'")
            .replace(['\u{201c}', '\u{201d}'], "\"")
    }

    /// Convert markdown to HTML via `pandoc`, stripping the `pandoc` document wrapper.
    fn md2html(md_src_path: &Path, filename: &str) -> String {
        let output = String::from_utf8_lossy(
            &std::process::Command::new("pandoc")
                .arg(md_src_path)
                .arg("--to")
                .arg("html")
                .arg("--mathjax")
                .arg("-s")
                .arg("--strip-comments")
                // ----------------------------------------------------
                // <<STYLE+TAG>>
                // ----------------------------------------------------
                // This is included here and not in `templates/markdown.html`
                // ----------------------------------------------------
                .arg("--css")
                .arg("/css/std.css")
                .arg("--highlight-style=zenburn")
                .output()
                .unwrap_or_else(|_| panic!("Failed to run `pandoc` for `{}`", filename))
                .stdout,
        )
        .to_string();
        // ----------------------------------------------------
        // * remove everything up until `<style>` (this include `<!DOCTYPE html>` and `<head>` and `<meta>`)
        // * then, add back `<head>`
        // * then, remove the trailing `</html>`
        // ----------------------------------------------------
        // this is to ensure that the formatting is consistent
        // across all notes
        // ----------------------------------------------------
        // <<STYLE+TAG>>
        // ----------------------------------------------------
        format!(
            "<head>{}",
            output[output.find("<style>").unwrap_or(0)..].trim_end_matches("</html>")
        )
    }
}
