#[macro_export]
/// Macro for quickly loading jsons, deserializing them into expected type T
macro_rules! json_template {
    ($file:expr) => {{
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