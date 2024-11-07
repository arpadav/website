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
/// A template for displaying a single project
pub struct ProjectTemplate {
    /// The name of the project
    pub name: String,
    /// The URL of the project
    pub url: String,
    /// The content of the project
    pub content: String,
    /// The "header/title" of the project
    pub header: ProjectHeader,
    /// The sidebar of the project page
    pub sidebar: SidebarType,
}

#[derive(Debug, Clone, Deserialize)]
/// A JSON file describing a project, used at the top of 
/// every project page
pub struct ProjectHeader {
    /// The title of the project
    pub title: String,

    #[serde(rename = "collab-type")]
    /// The type of collaboration
    /// 
    /// E.g. "Group", "Individual", etc
    pub collab_type: String,
    
    /// The status of the project
    /// 
    /// E.g. "Completed", "Hiatus", etc.
    pub status: String,

    #[serde(deserialize_with = "empty_as_none")]
    /// The description of the project
    pub desc: Option<String>,

    #[serde(deserialize_with = "empty_as_none")]
    /// The date of the project. General, year/month
    pub date: Option<String>,

    #[serde(deserialize_with = "empty_as_none")]
    /// The location of the project
    pub location: Option<String>,

    #[serde(default)]
    /// The list of repositories associated with the project, if any
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