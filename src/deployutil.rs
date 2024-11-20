// --------------------------------------------------
// external
// --------------------------------------------------
use std::io::Write;
use std::sync::{Arc, RwLock};

// --------------------------------------------------
// constants / statics
// --------------------------------------------------
static BRACKET_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"<(.*?)>").unwrap());
pub static DEPLOYMENT_MAP: LazyLock<DeploymentMap> = LazyLock::new(|| match DeploymentMapInner::from_static() {
    Ok(m) => DeploymentMap { inner: Arc::new(RwLock::new(m)) },
    Err(e) => panic!("{}", e),
});

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
    fn new(input: impl Into<String>) -> Result<Self, fancy_regex::Error> {
        // --------------------------------------------------
        // get regex pattern, return if error
        // --------------------------------------------------
        let strinput: String = input.into();
        let (capnames, re) = PathPattern::init_regex(&strinput);
        let re = match Regex::new(&re) {
            Ok(re) => re,
            Err(e) => return Err(e),
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
        let text = text.replace(".", r"\.");
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
            .replacen(&replacement_string, 1, &format!("(?P<{}>.*)", item))
            // .replacen(&replacement_string, 1, &format!("(?P<{}>.+?)", item))
                .to_string();
            // --------------------------------------------------
            // store shorthand reference for subsequent replacements, if any
            // --------------------------------------------------
            if replacement_string.contains(&format!("<{}>", item)) {
                replacements.insert(item, format!(r"\k<{}>", item));
            }
        }
        // --------------------------------------------------
        // match all `<...>` that arent `(?P<...>.*)`, replace
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
        // let debug_print = destination.contains("<ext>");
        // let debug_print = true;
        let unique_items = PathPattern::get_uniq_bracketized(&destination);
        let res = self.captures
            .iter()
            .map(|(src, captures)| {
                let mut dst = destination.clone();
                unique_items
                    .iter()
                    .filter(|ci| captures.contains_key(*ci))
                    // .for_each(|ci| match debug_print {
                    //     true => panic!("{:?}", captures),
                    //     false => dst = dst.replace(&format!("<{}>", ci), captures.get(ci).unwrap()),
                    // });
                    .for_each(|ci| { dst = dst.replace(&format!("<{}>", ci), captures.get(ci).unwrap()); });
                (src.clone(), std::path::PathBuf::from(dst))
            })
            .collect();
        res
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

/// Wrapper for a [`DeploymentMapInner`] to enable syntactic sugar on reading and writing
pub struct DeploymentMap {
    inner: Arc<RwLock<DeploymentMapInner>>,
}
/// [`DeploymentMap`] implementation
impl DeploymentMap {
    /// Read
    pub fn r(&self) -> std::sync::RwLockReadGuard<'_, DeploymentMapInner> {
        self.inner.read().unwrap()
    }
    /// Write
    pub fn w(&self) -> std::sync::RwLockWriteGuard<'_, DeploymentMapInner> {
        self.inner.write().unwrap()
    }
}

/// Underlying struct for a deployment map
pub struct DeploymentMapInner {
    files: Vec<DeploymentFile>,
}
/// [`DeploymentMapInner`] implementation
impl DeploymentMapInner {
    /// Creates a new [`DeploymentMapInner`]
    fn new(files: Vec<DeploymentFile>) -> Self {
        Self { files }
    }

    /// Reads in deployment map json, stores into static variable
    fn from_static() -> Result<DeploymentMapInner, std::io::Error> {
        // --------------------------------------------------
        // open deployment map json
        // --------------------------------------------------
        let contents: HashMap<String, Vec<(String, String)>> = match std::fs::read_to_string(std::path::PathBuf::from(crate::DEPLOYMENT_MAP_JSON)) {
            Ok(contents) => match serde_json::from_str::<HashMap<String, serde_json::Value>>(&contents) {
                Ok(mappings) => mappings
                    .iter()
                    .map(|(k, v)| {
                        let ordered: Vec<(&String, &serde_json::Value)> = v.as_object().unwrap().into_iter().collect();
                        let ordered: Vec<(String, String)> = ordered.into_iter().map(|(k, v)| (k.clone(), v.as_str().unwrap().to_string())).collect();
                        (k.clone(), ordered)
                    })
                    .collect(),
                Err(e) => return Err(e.into()),
            },
            Err(e) => return Err(e),
        };
        // --------------------------------------------------
        // get include and exclude files
        // --------------------------------------------------
        let include_contents = contents.get("include").expect("Failed to find `include` in deployment map");
        let exclude_contents = contents.get("exclude").expect("Failed to find `exclude` in deployment map");
        // --------------------------------------------------
        // convert to deployment maps, and take difference, return
        // --------------------------------------------------
        let include_map = DeploymentMapInner::from(include_contents);
        let exclude_map = DeploymentMapInner::from(exclude_contents);
        Ok(include_map - exclude_map)
    }

    /// Gets a copy of the source/destination, depending on the input [`DeploymentFileType`]
    pub fn pop(&self, f: DeploymentFileType) -> Option<&PathBuf> {
        match f {
            DeploymentFileType::Source(src) => self.files.iter().find(|x| x.src == **src).map(|x| &x.dst),
            // DeploymentFileType::Destination(dst) => self.files.iter().find(|x| x.dst == **dst).map(|x| &x.src),
        }
    }

    /// Checks if a file exists or not
    pub fn exists(&self, f: DeploymentFileType) -> bool {
        match f {
            DeploymentFileType::Source(src) => self.files.iter().any(|x| x.src == **src),
            // DeploymentFileType::Destination(dst) => self.files.iter().any(|x| x.dst == **dst),
        }
    }

    /// Marks a file as deployed
    /// 
    /// This is used in [`crate::deploy!`]
    pub fn mark(&mut self, dst: PathBuf) {
        self.files
            .iter_mut()
            .find(|x| x.dst == dst)
            .map(|x| x.deployed = true);
    }

    /// Iterates over files that have not been deployed
    pub fn not_deployed(&mut self) -> impl Iterator<Item = &mut DeploymentFile> {
        self.files
            .iter_mut()
            .filter(|x| !x.deployed)
    }
}
/// [`DeploymentMapInner`] implementation of [`From`] for [`Vec<(String, String)>`]
impl From<&Vec<(String, String)>> for DeploymentMapInner {
    fn from(input: &Vec<(String, String)>) -> Self {
        input
            .iter()
            .map(|(src, dst)| -> Option<Vec<(PathBuf, PathBuf)>> {
                let src = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(src);
                let dst = std::path::PathBuf::from(crate::DEPLOY_DIR.get().expect("`DEPLOY_DIR` is not initialized")).join(dst);
                match PathPattern::new(src.display().to_string()) {
                    Ok(pattern) => Some(pattern.map(dst.display().to_string())),
                    Err(err) => {
                        println!("Error creating `PathPattern`: {}", err);
                        None
                    },
                }
            })
            .filter_map(|x| x)
            .flatten()
            .collect::<Vec<(PathBuf, PathBuf)>>()
            .iter()
            // --------------------------------------------------
            // reverse and remove all duplicates from the beginning
            // therefore in `deployment-map.json`, you can think about
            // it like general cases come first, and specific cases 
            // overwrite them afterwards
            // --------------------------------------------------
            // <<STYLE+TAG>>
            // --------------------------------------------------
            .rev()
            .scan(HashSet::new(), |seen, (src, dst)| {
                if seen.insert(src) { Some(Some((src.clone(), dst.clone()))) } else {  Some(None) }
            })
            .filter_map(|x| x)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
            .into()
    }
}
/// [`DeploymentMapInner`] implementation of [`From`] for [`Vec<(PathBuf, PathBuf)>`]
impl From<Vec<(PathBuf, PathBuf)>> for DeploymentMapInner {
    fn from(input: Vec<(PathBuf, PathBuf)>) -> Self {
        input
            .into_iter()
            .map(|(src, dst)| DeploymentFile::new(src, dst))
            .collect::<Vec<DeploymentFile>>()
            .into()
    }
}
/// [`DeploymentMapInner`] implementation of [`From`] for [`Vec<DeploymentFile>`]
impl From<Vec<DeploymentFile>> for DeploymentMapInner {
    fn from(input: Vec<DeploymentFile>) -> Self {
        DeploymentMapInner::new(input)
    }
}
/// [`DeploymentMapInner`] implementation of [`std::ops::Sub`]
impl std::ops::Sub for DeploymentMapInner {
    type Output = DeploymentMapInner;
    fn sub(self, rhs: DeploymentMapInner) -> Self::Output {
        self.files
            .into_iter()
            .filter(|f| !rhs.files.contains(f))
            .collect::<Vec<DeploymentFile>>()
            .into()
    }
}
/// [`DeploymentMapInner`] implementation of [`std::ops::Deref`]
/// 
/// This makes for easy iteration
impl<'a> std::ops::Deref for DeploymentMapInner {
    type Target = Vec<DeploymentFile>;
    fn deref(&self) -> &Self::Target {
        &self.files
    }
}

#[derive(PartialEq)]
/// Underlying struct for a deployment file
pub struct DeploymentFile {
    pub src: PathBuf,
    pub dst: PathBuf,
    pub deployed: bool,
}
/// [`DeploymentFile`] implementation
impl DeploymentFile {
    /// Creates a new [`DeploymentFile`]
    fn new(src: PathBuf, dst: PathBuf) -> Self {
        Self {
            src,
            dst,
            deployed: false,
        }
    }

    /// Copies a file OR directory from `src` to `dst`
    pub fn copy(&mut self) -> std::io::Result<()> {
        if let Some(parent) = self.dst.parent() { std::fs::create_dir_all(parent)?; }
        match self.dst.extension() {
            None => {
                if !self.dst.exists() { std::fs::create_dir(&self.dst)?; }
                copy_dir(&self.src, &self.dst)?;
            },
            Some(_) => {
                std::fs::File::create(&self.dst)?.write_all(&[])?;
                std::fs::copy(&self.src, &self.dst)?;
            },
        }
        self.deployed = true;
        Ok(())
    }
}

/// Recursively copies a directory from `src` to `dst`
fn copy_dir(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?.path();
        let target = dst.join(entry.file_name().unwrap());
        match entry.is_dir() {
            true => copy_dir(&entry, &target)?,
            false => { let _ = std::fs::copy(entry, target)?; },
        };
    }
    Ok(())
}

/// For querying deployment map
pub enum DeploymentFileType<'a> {
    Source(&'a PathBuf),
    // Destination(&'a PathBuf),
}

/// Deploys a page
/// 
/// To be used by [`crate::deploy!`] macro only
pub(crate) fn deploy_fn(path: &PathBuf, page_to_render: &impl askama::Template, desc: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() { std::fs::create_dir_all(parent)?; }
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;
    let mut output = String::new();
    page_to_render
        .render_into(&mut output)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to render {}: {}", desc, e)))?;
    std::io::Write::write_all(&mut file, output.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_main() -> () {
        let _ = crate::DEPLOY_DIR.set(std::path::PathBuf::from("/home/arpadav/repos/website/deploy/dev"));
        assert!(DeploymentMapInner::from_static().is_ok());
    }

    #[test]
    fn match_escape_period() -> () {
        let regex_str = r"/home/arpadav/repos/website/content/notes/(?P<cat>.*)/(?P<typ>.*)/(?P<title>.*)\.(?P<ext>.*)";
        // let regex_st2 = r"/home/arpadav/repos/website/content/notes/(?P<cat>.[^/]+)/(?P<typ>.[^/]+)/(?P<title>.[^/]+)\.(?P<ext>.[^/]+)";
        let path = "/home/arpadav/repos/website/content/notes/academic/2021 - ECE 558/Voros_Arpad_ECE558_proj3.pdf";
        let regex = fancy_regex::Regex::new(regex_str).unwrap();
        let captures = regex.captures(path).unwrap().unwrap();
        println!("{:?}:", captures.name("cat").unwrap().as_str());
        println!("{:?}:", captures.name("typ").unwrap().as_str());
        println!("{:?}:", captures.name("title").unwrap().as_str());
        println!("{:?}:", captures.name("ext").unwrap().as_str());
        // println!("{:?}", captures);
    }
}