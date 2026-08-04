// --------------------------------------------------
// local
// --------------------------------------------------
use crate::prelude::*;

// --------------------------------------------------
// statics
// --------------------------------------------------
/// The visibility switches + deploy location for the CV
///
/// Tied to: `templates/cv.json`. The CV is referenced in several
/// places (home tab, navbar, prose links) and lives as a static file
/// deployed outside the deployment map, so one config drives them all
pub static CV: LazyLock<CvConfig> = crate::lazy_json_template!("cv.json");

#[derive(Deserialize)]
/// CV configuration, describing how the CV is reachable and where it lives
///
/// The two switches are independent, and together cover three states:
///
/// * `cv_findable` + `cv_url_servable`: fully public
/// * `cv_url_servable` only: unlisted -- the URL serves, nothing links to it
/// * neither: gone -- the URL 404s and nothing links to it
pub struct CvConfig {
    #[serde(default = "CvConfig::enabled")]
    /// Whether the CV can be found from the site. Default is true
    ///
    /// When false, every CV link is suppressed (home tab, navbar, the
    /// prose references), so nothing on the site names the CV. Whether
    /// the file itself still serves is the separate
    /// [`CvConfig::cv_url_servable`] switch. The source `static/cv.pdf`
    /// is never touched, so it remains in this repo either way
    pub cv_findable: bool,

    #[serde(default = "CvConfig::enabled")]
    /// Whether the CV URL serves the file. Default is true
    ///
    /// When true, the CV is published at `deploy_path`, so `/cv.pdf`
    /// resolves for anyone handed the URL -- independently of whether
    /// anything links to it. When false, the deployed copy is deleted
    /// and the URL 404s
    ///
    /// Cannot be false while `cv_findable` is true: that would leave
    /// every CV link pointing at a file that is not there
    pub cv_url_servable: bool,

    /// The CV link identity
    ///
    /// Must equal the home-tab `id` and the navbar `disp` of the CV
    /// entries. Changing it without updating both JSONs silently stops
    /// the link from being suppressed
    pub id: String,

    /// The CV's path under the deploy folder, deleted when not servable
    ///
    /// E.g. `cv.pdf` -> `<deploy>/cv.pdf`
    pub deploy_path: String,
}
/// [`CvConfig`] implementation
impl CvConfig {
    /// Serde default for both visibility switches: on
    ///
    /// Both switches read as permissions, so an absent one grants it and
    /// the CV stays fully public -- omitting them entirely is the same
    /// as the site before either existed
    const fn enabled() -> bool {
        true
    }

    /// Panics when the two switches contradict each other
    ///
    /// A findable CV must be servable, otherwise every link rendered
    /// into the site resolves to a 404. Checked once, up front, rather
    /// than silently overriding one switch with the other
    pub fn validate(&self) {
        assert!(
            !(self.cv_findable && !self.cv_url_servable),
            "`cv.json`: `cv_findable` is true while `cv_url_servable` is false, \
             which would link `/{}` from the site without deploying it",
            self.deploy_path
        );
    }

    /// Whether a CV link with this `id` should be suppressed
    ///
    /// True only when the CV is unfindable and the `id` matches (the
    /// home tab `id` / navbar `disp`). Lets each render site read
    /// forward: `filter(|x| !CV.suppresses(&x.id))`
    pub fn suppresses(&self, id: &str) -> bool {
        !self.cv_findable && id == self.id
    }

    /// Deletes the CV from the deploy folder when the URL should not serve
    ///
    /// `cv.pdf` reaches the deploy folder via `rsync static/` in
    /// `build.sh` (NOT the deployment map), which runs before this
    /// binary. So when unservable, it is deleted from the output here.
    /// The source `static/cv.pdf` is left untouched
    pub fn delete_if_unservable(&self) {
        // --------------------------------------------------
        // nothing to do if the url should serve the cv
        // --------------------------------------------------
        if self.cv_url_servable {
            return;
        }
        // --------------------------------------------------
        // remove the deployed copy, tolerating its absence;
        // warn (never abort the build) on any other error
        // --------------------------------------------------
        let dir = crate::DEPLOY_DIR
            .get()
            .expect("`DEPLOY_DIR` is not initialized");
        let path = dir.join(&self.deploy_path);
        if let Err(e) = std::fs::remove_file(&path) {
            if e.kind() != std::io::ErrorKind::NotFound {
                eprintln!(
                    "Warning: failed to remove unservable CV `{}`: {e}",
                    path.display()
                );
            }
        }
    }
}
