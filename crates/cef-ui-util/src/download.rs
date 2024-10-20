use crate::{download_file, extract_bz2};
use anyhow::Result;
use std::{fs::create_dir_all, path::Path};
use std::path::PathBuf;

const CEF_VERSION: &str = "130.1.2+g48f3ef6+chromium-130.0.6723.44_windows64";

pub fn get_cef_artifacts_name() -> String {
    format!("cef_binary_{}_minimal", CEF_VERSION)
}

pub fn get_cef_url() -> String {
    format!(
        "https://cef-builds.spotifycdn.com/cef_binary_{}_minimal.tar.bz2",
        CEF_VERSION,
    )
}

/// Downloads the tarball, untars it, and decompresses it. If the
/// target directory exists, then this function does nothing.
pub fn download_and_extract_cef(dir: &Path) -> Result<CefDir> {
    let url = get_cef_url();

    if !dir.exists() {
        create_dir_all(dir)?;
    }

    let extracted = dir.join(get_cef_artifacts_name());

    if extracted.exists() {
        return Ok(CefDir { path: extracted });
    }

    let cef_file_name = format!("{}.tar.bz2", get_cef_artifacts_name());

    let path = dir.join(cef_file_name);

    if !path.exists() {
        download_file(&url, &path)?;
    }

    extract_bz2(&path, dir)?;

    Ok(CefDir { path: extracted })
}

pub struct CefDir {
    path: PathBuf,
}

impl CefDir {
    pub fn get_release_dir(&self) -> PathBuf {
        self.path.join("Release")
    }

    pub fn get_resources_dir(&self) -> PathBuf {
        self.path.join("Resources")
    }
}
