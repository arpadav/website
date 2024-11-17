#[macro_export]
/// Macro to constructing a page [`crate::primitives::Page`] and then deploying
macro_rules! deploy {
    ($page:ident, $constructor:path) => {
        let $page = <$constructor>::to_page();
        assert!(deployutil::DEPLOYMENT_MAP.r().exists(deployutil::DeploymentFileType::Source(&$page.src)), concat!( stringify!($page), " not found in deployment map" ));
        $crate::page_deploy!($page, stringify!($page));
    };
}

#[macro_export]
/// Macro to deploy a page [`crate::primitives::Page`]
macro_rules! page_deploy {
    ($page:expr, $desc:expr) => {
        let template_path = PathBuf::from(&$page.src);
        let binding = $crate::deployutil::DEPLOYMENT_MAP.r();
        let deployment_path = binding.pop($crate::deployutil::DeploymentFileType::Source(&template_path)).unwrap_or_else(|| panic!("Failed to find {} in deployment map", template_path.display())).clone();
        drop(binding);
        $crate::deployutil::DEPLOYMENT_MAP.w().mark(deployment_path.clone());
        let _ = $crate::deployutil::deploy_fn(&deployment_path, &$page.page, $desc).expect(&format!("Failed to deploy {}", $desc));
    };
}

#[macro_export]
/// Creates a blank page with no fields, other than a [`crate::primitives::SidebarType::GatorOnly`] sidebar
macro_rules! blank_page {
    ($template_path:expr, $struct_name:ident) => {
        #[derive(askama::Template)]
        #[template(path = $template_path)]
        #[doc = concat!("Template for ", $template_path)]
        pub struct $struct_name {
            sidebar: $crate::primitives::SidebarType
        }

        #[doc = concat!(" [`", stringify!($struct_name), "`] implementation of [`crate::prelude::Create`]")]
        impl $crate::prelude::Create for $struct_name {
            fn create() -> Self {
                Self {
                    sidebar: Default::default()
                }
            }
        }

        #[doc = concat!(" [`", stringify!($struct_name), "`] implementation of [`crate::prelude::SourcePath`]")]
        impl $crate::prelude::SourcePath<$struct_name> for $struct_name {
            fn src_path() -> std::path::PathBuf {
                [crate::TEMPLATES_DIR, "/", $template_path].concat().into()
            }
        }
    };
}

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

#[macro_export]
/// To get the items after "content" from CARGO_MANIFEST_DIR
/// 
/// Used in templates
macro_rules! url_relative_content {
    ($url:expr) => {{
        let url_absolute = std::path::Path::new($url).canonicalize().unwrap();
        let url_absolute = url_absolute.to_str().unwrap();
        url_absolute.replace(concat!( env!("CARGO_MANIFEST_DIR"), "/content" ), "").to_string()
    }};
}

#[macro_export]
/// To get the items after CARGO_MANIFEST_DIR
/// 
/// Used in templates
macro_rules! url_relative_manifest {
    ($url:expr) => {{
        let url_absolute = std::path::Path::new($url).canonicalize().unwrap();
        let url_absolute = url_absolute.to_str().unwrap();
        url_absolute.replace(env!("CARGO_MANIFEST_DIR"), "").to_string()
    }};
}
