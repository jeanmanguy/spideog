use color_eyre::{Report, Result};

pub fn setup_error_hook() -> Result<(), Report> {
    color_eyre::config::HookBuilder::default()
        .issue_url("https://github.com/jeanmanguy/spideog/issues/new")
        .add_issue_metadata("version", crate_version!())
        .issue_filter(|kind| match kind {
            color_eyre::ErrorKind::NonRecoverable(_) => true,
            color_eyre::ErrorKind::Recoverable(_) => false,
        })
        .install()
}
