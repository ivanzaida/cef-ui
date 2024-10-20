use std::env::var;
use std::path::PathBuf;
use crate::{copy_files, download_and_extract_cef, get_cef_artifacts_dir, get_cef_cef_dir, get_cef_target_dir, CefDir};
use anyhow::Result;

pub const CEF_RESOURCES_DIR : &str = "CEF_RESOURCES_DIR";


/// Call this in your binary crate's build.rs
/// file to properly link against CEF.
pub fn link_cef() -> Result<()> {
    #[cfg(not(target_os = "windows"))]
    {
        panic!("This crate only supports Windows.");
    }

    let artifacts_dir = get_cef_artifacts_dir()?;
    let cef_dir = download_and_extract_cef(&artifacts_dir)?;

    let dst = copy_cef_windows(&cef_dir)?;
    println!("cargo:rustc-link-search=native={}", &dst.display());

    Ok(())
}


/// Copy the CEF files to the target directory on Windows.
#[allow(dead_code)]
fn copy_cef_windows(cef_dir: &CefDir) -> Result<PathBuf> {
    let profile = var("PROFILE")?;
    let dst = get_cef_target_dir(&profile)?;
    let release = cef_dir.get_release_dir();
    let resources = cef_dir.get_resources_dir();

    let resources_dir = var(CEF_RESOURCES_DIR).unwrap_or(String::new());
    // Copy the CEF binaries.
    copy_files(&release, &dst)?;
    copy_files(&resources, &dst.join(&resources_dir))?;

    Ok(dst)
}

/// Call this in your binary helper crate's build.rs file to
/// properly link against the CEF sandbox static library.
pub fn link_cef_helper() -> Result<()> {
    let artifacts_dir = get_cef_artifacts_dir()?;
    let cef_dir = get_cef_cef_dir()?;

    // Download and extract the CEF binaries.
    download_and_extract_cef(&artifacts_dir)?;

    // Link against the CEF sandbox static library.
    println!("cargo:rustc-link-search=native={}", cef_dir.display());
    println!("cargo:rustc-link-lib=static=cef_sandbox");

    // We must also link against the macOS sandbox libary.
    println!("cargo:rustc-link-lib=sandbox");

    Ok(())
}
