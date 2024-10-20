#![allow(non_snake_case)]

// --------------------------------------------------
// external
// --------------------------------------------------
use serde::Deserialize;
use std::sync::LazyLock;

// --------------------------------------------------
// local
// --------------------------------------------------
use crate::prelude::*;

// --------------------------------------------------
// statics
// --------------------------------------------------
// <<STYLE+TAG>>>
// --------------------------------------------------
pub static ALL_TABS: LazyLock<Vec<Tab>> = LazyLock::new(|| {
    // --------------------------------------------------
    // about me
    // --------------------------------------------------
    let ABOUT_ME_TAB = Tab {
        head: TabHead::default("about"),
        body: Some(TabBodyType::AboutMe),
    };
    // --------------------------------------------------
    // contact
    // --------------------------------------------------
    let CONTACT_TAB = Tab {
        head: TabHead::default("contact"),
        body: Some(TabBodyType::Contact),
    };
    // --------------------------------------------------
    // socials
    // --------------------------------------------------
    let SOCIALS_TAB = Tab {
        head: TabHead::default("socials"),
        body: Some(TabBodyType::Socials(crate::json_template!("homepage/tabs/socials-tab-links.json"))),
    };
    // --------------------------------------------------
    // remaining links
    // --------------------------------------------------
    let REMAINING_TABS: Vec<TabHead> = crate::json_template!("homepage/tabs/tab-links.json");
    let mut REMAINING_TABS = REMAINING_TABS.into_iter().map(Tab::from).collect();
    // --------------------------------------------------
    // order them and return
    // --------------------------------------------------
    let mut tabs_ordered = vec![
        ABOUT_ME_TAB,
        CONTACT_TAB,
        SOCIALS_TAB,
    ];
    tabs_ordered.append(&mut REMAINING_TABS);
    tabs_ordered
});

#[derive(Clone)]
/// The tabs on the home page which have custom bodies. Please refer to
/// `templates/homepage/index.html` for more information, since all
/// custom body logic is implemented there
pub enum TabBodyType {
    /// The "about me" tab
    AboutMe,
    /// The contact tab
    Contact,
    /// The tab with links to socials
    /// 
    /// See [`SocialTabLink`]
    Socials(Vec<SocialTabLink>),
}

#[derive(Clone)]
//// A struct describing a tab on the home page
/// 
/// Please refer to `templates/homepage/index.html` for more information
pub struct Tab {
    /// Required text/title of the tab
    pub head: TabHead,
    /// Optional expanding body
    /// 
    /// See [`TabBodyType`]
    pub body: Option<TabBodyType>,
}
/// [`Tab`] implmentation of [`From`] for [`TabHead`]
impl From<TabHead> for Tab {
    fn from(input: TabHead) -> Self {
        Tab {
            head: input,
            body: None,
        }
    }
}

#[derive(Clone, Deserialize)]
//// A struct describing a tab head on the home page
pub struct TabHead {
    /// The id of the tab, used in `div id={{ id }}`
    pub id: String,
    /// A provided link, if any
    pub link: Option<String>,
    /// The HTML tag of the [`TabHead::title`]
    pub tag: String,
    /// The title of the tab, used in `<{{ tag }}>{{ title }}</{{ tag }}>`
    pub title: String,
}
/// [`TabHead`] implementation
impl TabHead {
    /// Renders the title of the tab, using appropriate HTML tags
    fn render_title(&self) -> String {
        format!("<{}>{}</{}>", self.tag, self.title, self.tag)
    }
    /// Creates a new [`TabHead`] with the default values
    fn default(name: &str) -> Self {
        TabHead {
            id: name.into(),
            link: None,
            tag: "h2".into(),
            title: name.into(),
        }
    }
}
/// [`TabHead`] implementation of [`Render`]
impl Render for TabHead {
    /// Renders the entire tab head, with or without a link
    fn render(&self) -> String {
        let title_html = self.render_title();
        match &self.link {
            Some(link) => format!("<a href=\"{}\">{}</a>", link, title_html),
            None => format!("{}", title_html),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
/// Social link structure
/// 
/// Tied to: `templates/homepage/tabs/socials-tab-links.json`
pub struct SocialTabLink {
    /// The name of the social media website
    pub name: String,
    /// The URL of the link
    pub url: String,
    /// The icon of the social media site
    pub img_src: String,
    /// The icon of the social media site when hovered
    pub img_hvr_src: String,
}