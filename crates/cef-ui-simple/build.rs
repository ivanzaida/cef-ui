use std::env::{current_dir, var};
use std::path::PathBuf;
use anyhow::Result;
use cef_ui_util::link_cef;

fn main() -> Result<()> {
    link_cef()?;

    println!("cargo:rustc-link-search=native={}", &get_out_dir()?.display());
    Ok(())
}

fn get_out_dir() -> Result<PathBuf> {
    let profile = var("PROFILE")?;

    let mut out_dir = current_dir()?;
    out_dir.push("target");
    out_dir.push(profile);

    let bin = var("BIN_SEARCH_DIR");

    if let Ok(bin) = bin {
        out_dir.push(bin);
    }


    Ok(out_dir)
}