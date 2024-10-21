#[derive(Debug)]
/// The type of sidebar, and the contents to display
pub enum SidebarType {
    Projects,
}


use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct Mappings {
    mappings: HashMap<String, String>,
}

struct PatternedPath {
    contents: Vec<PathPattern>
}
impl From<std::path::PathBuf> for PatternedPath {
    fn from(p: std::path::PathBuf) -> Self {
        Self::from(p.to_str().unwrap())
    }
}
impl From<&str> for PatternedPath {
    fn from(s: &str) -> Self {
        let split = s.split('/').collect::<Vec<&str>>();
        let contents = split
            .into_iter()
            .map(|s| match s {
                "*" => PathPattern::ForAll,
                "+" => PathPattern::ForOne,
                _ if s.contains('+') => {
                    let split = s.split('+').collect::<Vec<&str>>();
                    PathPattern::All(split[0].to_string(), split[1].to_string())
                },
                _ if s.contains('*') => {
                    let split = s.split('*').collect::<Vec<&str>>();
                    PathPattern::One(split[0].to_string(), split[1].to_string())
                },
                _ if s.starts_with('<') && s.ends_with('>') => PathPattern::Captured(s[1..s.len() - 1].to_string()),
                _ => PathPattern::Static(s.to_string()),
            })
            .collect();

        Self { contents }
    }
}
impl PatternedPath {
    fn glob(&self, prior: std::path::PathBuf) -> (Vec<std::path::PathBuf>, Option<HashMap<String, String>>) {
        let mut captures = HashMap::new();
        let mut recurse_results = Vec::new();
        for pattern in &self.contents {
            let (mut res, capts) = pattern.glob(prior.clone());
            recurse_results.append(&mut res);
            match capts {
                Some(c) => captures.extend(c),
                None => (),
            }
        }

        let mut new_contents = self.contents.clone();
        new_contents.remove(0);
        

        let mut results = Vec::new();
        for r in recurse_results {
            let (mut res, capts) = self.glob(r);
            results.append(&mut res);
            match capts {
                Some(c) => captures.extend(c),
                None => (),
            }
        }

        (results, Some(captures))
    }
}

#[derive(Debug, Clone)]
enum PathPattern {
    Static(String),
    ForAll,
    ForOne,
    All(String, String),
    One(String, String),
    Captured(String),
}
impl PathPattern {
    fn to_string(&self) -> String {
        match self {
            PathPattern::ForAll => "*".to_string(),
            PathPattern::ForOne => "+".to_string(),
            PathPattern::All(s1, s2) => format!("{}*{}", s1, s2),
            PathPattern::One(s1, s2) => format!("{}+{}", s1, s2),
            PathPattern::Static(s) => s.to_string(),
            PathPattern::Captured(s) => format!("<{}>", s),
        }
    }

    fn glob(&self, prior: std::path::PathBuf) -> (Vec<std::path::PathBuf>, Option<HashMap<String, String>>) {
        
        match self {

            // finds all entries, folder or file
            PathPattern::ForAll => {
                let result = prior
                    .read_dir()
                    .map(|entries| entries
                        .into_iter()
                        .filter_map(|entry| entry.ok())
                        .map(|entry| entry.path())
                        .collect()
                    )
                    .unwrap_or(Vec::new());
                (result, None)
            },
            
            // finds the first entry, folder or file
            PathPattern::ForOne => {
                let first = prior
                    .read_dir()
                    .map(|entries| entries
                        .into_iter()
                        .filter_map(|entry| entry.ok())
                        .map(|entry| entry.path())
                        .find(|entry| entry.is_dir() || entry.is_file())
                    )
                    .unwrap_or(None);
                match first {
                    Some(first) => (vec![first], None),
                    None => (Vec::new(), None)
                }
            },

            // globs with * pattern
            PathPattern::All(_, _) => {
                let result = glob::glob(&format!("{}/{}", prior.display(), self.to_string()))
                    .map(|iter| {
                        iter.filter_map(|entry| entry.ok())
                            .collect()
                    })
                    .unwrap_or_else(|_| Vec::new());
                (result, None)
            },
            
            // globs with * pattern, but gets the first entry
            PathPattern::One(s1, s2) => {
                let (globs, capts) = PathPattern::All(s1.to_string(), s2.to_string()).glob(prior.clone());
                (vec![globs[0].clone()], capts)
            },

            // returns static path, if it exists
            PathPattern::Static(s) => {
                let result = prior.join(s).exists().then(|| vec![prior.join(s)]).unwrap_or(Vec::new());
                (result, None)
            },

            // captures item
            PathPattern::Captured(s) => {
                let mut captures = HashMap::new();
                // glob all
                let (globs, _) = PathPattern::ForAll.glob(prior.clone());
                for glob in &globs {
                    let disp: String = glob.display().to_string();
                    let url_last = crate::url_last!(disp);
                    captures.insert(s.to_string(), url_last.to_string());
                }
                (globs, Some(captures))
            }
        }
    }
}

// fn parse_pattern(pattern: &str) -> (PathPattern, Option<String>) {
//     if pattern.contains('*') {
//         (PathPattern::Wildcard, None)
//     } else if let Some(capture) = pattern.strip_prefix('<').and_then(|s| s.strip_suffix('>')) {
//         (PathPattern::Captured(capture.to_string()), Some(capture.to_string()))
//     } else if pattern.contains('+') {
//         let base = pattern.strip_suffix('+').unwrap().to_string();
//         (PathPattern::Single(base), None)
//     } else {
//         (PathPattern::Static(pattern.to_string()), None)
//     }
// }

// fn map_paths(mappings: &Mappings) -> Result<HashMap<String, String>, String> {
//     let mut result = HashMap::new();

//     // Example folders and HTML files for demonstration
//     let categories = vec!["folder1", "folder2", "folder3"]; // Example folders
//     let html_files = vec![
//         ("folder1", vec!["file1.html"]),
//         ("folder2", vec!["file2.html", "file3.html"]), // Multiple files here to trigger an error
//         ("folder3", vec!["file4.html"]),
//     ];

//     for folder in categories {
//         // Check for HTML files in the folder
//         let files = html_files.iter().find(|(name, _)| name == &folder);

//         if let Some((_, files)) = files {
//             match files.len() {
//                 0 => return Err(format!("No HTML files found in folder {}", folder)),
//                 1 => {
//                     let html_file = &files[0];

//                     // Iterate over each mapping
//                     for (source_pattern, target_pattern) in &mappings.mappings {
//                         let (source_path, _) = parse_pattern(source_pattern);
//                         let target_path = target_pattern.replace("<folder>", folder);

//                         // Handle wildcards
//                         match source_path {
//                             PathPattern::Wildcard => {
//                                 let generated_source = source_pattern.replace("*", folder);
//                                 result.insert(generated_source, html_file.to_string());
//                                 result.insert(target_path.clone(), "index.html".to_string());
//                             }
//                             PathPattern::Static(src) => {
//                                 result.insert(src, html_file.to_string());
//                                 result.insert(target_path, "index.html".to_string());
//                             }
//                             _ => {} // Add additional handling as necessary
//                         }
//                     }
//                 }
//                 _ => return Err(format!("Multiple HTML files found in folder {}", folder)),
//             }
//         }
//     }

//     Ok(result)
// }

#[test]
fn main() {
    // // Simulating JSON input with dynamic mappings
    // let json_data = r#"
    // {
    //     "mappings": {
    //         "projects/categories/*/<folder>/+.html": "<folder>/index.html",
    //         "homepage/index.html": "index.html"
    //     }
    // }"#;

    // let mappings: Mappings = serde_json::from_str(json_data).expect("Failed to parse JSON");

    // match map_paths(&mappings) {
    //     Ok(mapped_paths) => {
    //         for (source, target) in mapped_paths {
    //             println!("Move {} to {}", source, target);
    //         }
    //     }
    //     Err(err) => eprintln!("Error: {}", err),
    // }

    let map_input = "projects/categories/*/<folder>/+.html";
    let prior = crate::TEMPLATES_DIR;
    let patterned_path = PatternedPath::from(map_input);
    let results = patterned_path.glob(std::path::PathBuf::from(prior));
    println!("{:?}", results);
}
