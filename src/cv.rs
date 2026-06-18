// --------------------------------------------------
// local
// --------------------------------------------------
use crate::prelude::*;

// --------------------------------------------------
// statics
// --------------------------------------------------
/// The visibility switch + deploy location for the CV
///
/// Tied to: `templates/cv.json`. The CV is referenced in several
/// places (home tab, navbar, prose links) and lives as a static file
/// deployed outside the deployment map, so one switch drives them all
pub static CV: LazyLock<CvConfig> = crate::lazy_json_template!("cv.json");

#[derive(Deserialize)]
/// CV configuration, describing whether the CV is shown and where it lives
pub struct CvConfig {
    #[serde(default)]
    /// Whether the CV is hidden. Default is false (shown)
    ///
    /// When true, every CV link is suppressed (home tab, navbar, the
    /// prose references) and the deployed file is deleted. The source
    /// `static/cv.pdf` is never touched, so it remains in this repo
    pub hidden: bool,

    /// The CV link identity
    ///
    /// Must equal the home-tab `id` and the navbar `disp` of the CV
    /// entries. Changing it without updating both JSONs silently stops
    /// the link from being suppressed
    pub id: String,

    /// The CV's path under the deploy folder, deleted when hidden
    ///
    /// E.g. `cv.pdf` -> `<deploy>/cv.pdf`
    pub deploy_path: String,
}
/// [`CvConfig`] implementation
impl CvConfig {
    /// Whether a CV link with this `id` should be suppressed
    ///
    /// True only when the CV is hidden and the `id` matches (the home
    /// tab `id` / navbar `disp`). Lets each render site read forward:
    /// `filter(|x| !CV.suppresses(&x.id))`
    pub fn suppresses(&self, id: &str) -> bool {
        self.hidden && id == self.id
    }

    /// Deletes the published CV from the deploy folder when hidden
    ///
    /// `cv.pdf` reaches the deploy folder via `rsync static/` in
    /// `build.sh` (NOT the deployment map), which runs before this
    /// binary. So when hidden, it is deleted from the output here. The
    /// source `static/cv.pdf` is left untouched
    pub fn delete_if_hidden(&self) {
        // --------------------------------------------------
        // nothing to do if the cv is shown
        // --------------------------------------------------
        if !self.hidden {
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
                    "Warning: failed to remove hidden CV `{}`: {e}",
                    path.display()
                );
            }
        }
    }
}
