// --------------------------------------------------
// constants / statics
// --------------------------------------------------
static BRACKET_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"<(.*?)>").unwrap());
pub static DEPLOYMENT_MAP: LazyLock<Vec<(PathBuf, PathBuf)>> = LazyLock::new(|| get_deployment_map().unwrap());

// --------------------------------------------------
// prelude
// --------------------------------------------------
use crate::prelude::*;

// --------------------------------------------------
// external
// --------------------------------------------------
use std::path::PathBuf;
use fancy_regex::Regex;
use std::collections::HashSet;

#[derive(Debug)]
/// Easy path pattern mapping from source to destination,
/// for deployment
pub struct PathPattern {
    captures: Vec<(PathBuf, HashMap<String, String>)>,
}
/// [`PathPattern`] implementation
impl PathPattern {
    fn new(input: impl Into<String>) -> Result<Self, anyhow::Error> {
        // --------------------------------------------------
        // get regex pattern, return if error
        // --------------------------------------------------
        let strinput: String = input.into();
        let (capnames, re) = PathPattern::init_regex(&strinput);
        let re = match Regex::new(&re) {
            Ok(re) => re,
            Err(e) => return Err(anyhow::anyhow!(e)),
        };
        // --------------------------------------------------
        // glob all
        // --------------------------------------------------
        let results = glob::glob(&PathPattern::init_glob(&strinput))
            .unwrap()
            .filter_map(|result| result.ok())
            .collect::<Vec<_>>();
        // --------------------------------------------------
        // regex to capture all in `re`
        // --------------------------------------------------
        let captures = results
            .iter()
            .map(|f| match re.captures(f.display().to_string().as_str()) {
                Ok(Some(caps)) => capnames
                    .iter()
                    .map(|n| match caps.name(n) {
                        Some(c) => Some((n.clone(), c.as_str().to_string())),
                        _ => None,
                    })
                    .filter(|x| x.is_some())
                    .map(|x| x.unwrap())
                    .collect::<HashMap<String, String>>(),
                _ => HashMap::new(),
            })
            .collect::<Vec<HashMap<String, String>>>();
        // --------------------------------------------------
        // zip and return
        // --------------------------------------------------
        Ok(Self { captures: results.into_iter().zip(captures).collect::<Vec<_>>() })
    }

    /// Gets `glob` string from input path
    fn init_glob(text: &str) -> String {
        let text = BRACKET_RE.replace_all(text, "*");
        text.replace("**", "*").into()
    }

    /// Gets `regex` string from input path
    fn init_regex(text: &str) -> (HashSet<String>, String) {
        let unique_items = PathPattern::get_uniq_bracketized(text);
        // --------------------------------------------------
        // map for capture groups with group reference numbers
        // --------------------------------------------------
        let mut replacements = HashMap::new();
        let mut replacement_string = text.to_string();
        for item in &unique_items {
            // --------------------------------------------------
            // replace the first instance with the capture group
            // --------------------------------------------------
            let first_instance_re = Regex::new(&format!(r"<{}>", item)).unwrap();
            replacement_string = first_instance_re
                .replacen(&replacement_string, 1, &format!("(?P<{}>.+?)", item))
                .to_string();
            // --------------------------------------------------
            // store shorthand reference for subsequent replacements, if any
            // --------------------------------------------------
            if replacement_string.contains(&format!("<{}>", item)) {
                replacements.insert(item, format!(r"\k<{}>", item));
            }
        }
        // --------------------------------------------------
        // match all `<...>` that arent `(?P<...>.+?)`, replace
        // with short hand ref numbers
        // --------------------------------------------------
        for (item, shorthand) in &replacements {
            let remaining_instance_re = Regex::new(&format!(r"(?<!\(\?P)<{}>", item)).unwrap();
            replacement_string = remaining_instance_re.replace_all(&replacement_string, shorthand).to_string();
        }
        // --------------------------------------------------
        // replace all `*` with `[^/]+` to ensure multiple
        // characters are matched, never none
        // --------------------------------------------------
        (unique_items.clone(), replacement_string.replace("*", "[^/]+"))
    }

    /// Map sources to destination
    fn map(&self, destination: impl Into<String>) -> Vec<(PathBuf, PathBuf)> {
        let destination = destination.into();
        let unique_items = PathPattern::get_uniq_bracketized(&destination);
        self.captures
            .iter()
            .map(|(src, captures)| {
                let mut dst = destination.clone();
                unique_items
                    .iter()
                    .filter(|ci| captures.contains_key(*ci))
                    .for_each(|ci| { dst = dst.replace(&format!("<{}>", ci), captures.get(ci).unwrap()) });
                (src.clone(), std::path::PathBuf::from(dst))
            })
            .collect()
    }

    /// Captures all unique bracketed items
    /// 
    /// E.g. `<folder>` -> "folder", collecting all unique
    /// names into a hashset
    pub fn get_uniq_bracketized(text: &str) -> HashSet<String> {
        // --------------------------------------------------
        // regex to capture everything between `<...>` as unique
        // capture groups
        // --------------------------------------------------
        let mut unique_items = HashSet::new();
        BRACKET_RE
            .captures_iter(text)
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .map(|x| x.get(1))
            .filter(|x| x.is_some())
            .for_each(|x| { let _ = unique_items.insert(x.unwrap().as_str().to_string()); });
        unique_items
    }
}

fn get_deployment_map() -> Result<Vec<(PathBuf, PathBuf)>, anyhow::Error> {
    // --------------------------------------------------
    // open deployment map json
    // --------------------------------------------------
    let contents = match std::fs::read_to_string(std::path::PathBuf::from(crate::DEPLOYMENT_MAP)) {
        Ok(contents) => match serde_json::from_str::<std::collections::HashMap<String, String>>(&contents) {
            Ok(mappings) => mappings,
            Err(e) => return Err(anyhow::anyhow!(e)),
        },
        Err(e) => return Err(anyhow::anyhow!(e)),
    };
    let deployment_dir = match crate::DEPLOY_DIR.get() {
        Some(dir) => dir,
        None => return Err(anyhow::anyhow!("DEPLOY_DIR is not initialized")),
    };
    // --------------------------------------------------
    // collect all mappings into vec and return
    // --------------------------------------------------
    Ok(contents
        .iter()
        .map(|(src, dst)| -> Option<Vec<(PathBuf, PathBuf)>> {
            let src = std::path::PathBuf::from(crate::TEMPLATES_DIR).join(src);
            let dst = std::path::PathBuf::from(deployment_dir).join(dst);
            match PathPattern::new(src.display().to_string()) {
                Ok(pattern) => Some(pattern.map(dst.display().to_string())),
                Err(err) => {
                    println!("Error creating `PathPattern`: {}", err);
                    None
                },
            }
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .flatten()
        .collect::<Vec<(PathBuf, PathBuf)>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_main() -> () {
        let _ = crate::DEPLOY_DIR.set(std::path::PathBuf::from("deploy-debug"));
        assert!(get_deployment_map().is_ok());
    }
}