#[macro_export]
/// Macro for quickly loading jsons, deserializing them into expected type T
macro_rules! json_template {
    // --------------------------------------------------
    // if the input is a variable, must be able to stringify
    // --------------------------------------------------
    ($file:ident) => {{
        let file_string = String::from($file);
        let path = match file_string.contains($crate::TEMPLATES_DIR) {
            true => std::path::Path::new(&file_string),
            false => &std::path::Path::new($crate::TEMPLATES_DIR).join(file_string),   
        };
        let file = std::fs::File::open(&path).expect(format!("Failed to open `{}`", path.display()).as_str());
        serde_json::from_reader(file).expect(format!("Failed to parse `{}`", path.display()).as_str())
    }};
    // --------------------------------------------------
    // if the input is a literal, e.g. "foo.json"
    // --------------------------------------------------
    // this **SHOULD** always be relative to templates dir, 
    // but could change?
    // --------------------------------------------------
    ($file:literal) => {{
        let path = std::path::Path::new($crate::TEMPLATES_DIR).join($file);
        let file = std::fs::File::open(&path).expect(format!("Failed to open `{}`", path.display()).as_str());
        serde_json::from_reader(file).expect(format!("Failed to parse `{}`", path.display()).as_str())
    }};
}

#[macro_export]
/// Macro for quickly loading templates as [`LazyLock<T>`](std::sync::LazyLock)
macro_rules! lazy_json_template {
    ($file:expr) => {
        std::sync::LazyLock::new(|| { crate::json_template!($file) })
    };
}

#[macro_export]
/// To quickly split a URL into its components, removing all empty strings and ignoring the leading `://`
/// 
/// Is helper function. Not expecting to use this elsewhere
macro_rules! url_split {
    ($url:expr) => {{
        let url = $url.trim_start_matches("://");
        url.split('/').filter(|x| !x.is_empty()).collect::<Vec<&str>>()
    }};
}

#[macro_export]
/// To quickly get an index from a url
/// 
/// Used in templates
macro_rules! url_at {
    ($url:expr, $index:expr) => {
        url_split!($url)[$index]
    };
}

#[macro_export]
/// To quickly get the last item in a url
/// 
/// Used in templates
macro_rules! url_last {
    ($url:expr) => {{
        let url_split: Vec<&str> = crate::url_split!($url);
        url_split[url_split.len() - 1]
    }};
}