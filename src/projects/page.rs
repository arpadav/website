// --------------------------------------------------
// external
// --------------------------------------------------
use serde::Deserialize;

// --------------------------------------------------
// local
// --------------------------------------------------
use crate::prelude::*;

#[derive(Debug, Clone, Template)]
#[template(path = "projects/project-template.html")]
pub struct ProjectTemplate {
    pub name: String,
    pub url: String,
    pub content: String,
    pub header: ProjectHeader,
    pub sidebar: SidebarType,
}

#[derive(Debug, Clone, Deserialize)]
/// A JSON file describing a project, used at the top of 
/// every project page
pub struct ProjectHeader {
    pub title: String,
    #[serde(rename = "collab-type")]
    pub collab_type: String,
    pub status: String,
    #[serde(deserialize_with = "empty_as_none")]
    pub desc: Option<String>,
    #[serde(deserialize_with = "empty_as_none")]
    pub date: Option<String>,
    #[serde(deserialize_with = "empty_as_none")]
    pub location: Option<String>,
    #[serde(default)]
    pub repos: Vec<String>,
}

/// Deserializes an empty string into [`None`]
fn empty_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.filter(|s| !s.is_empty()))
}