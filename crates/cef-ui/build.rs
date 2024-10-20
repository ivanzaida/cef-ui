use anyhow::Result;
use cef_ui_util::{download_and_extract_cef, get_cef_artifacts_dir};

fn main() -> Result<()> {
    let artifacts_dir = get_cef_artifacts_dir()?;

    let cef_dir = download_and_extract_cef(&artifacts_dir)?;

    #[cfg(not(target_os = "windows"))]
    {
        panic!("This crate only supports Windows.");
    }

    // Linker flags on x86_64 Windows.
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    {
        // Link statically to the CEF sandbox.
        println!("cargo:rustc-link-search=native={}", &cef_dir.get_release_dir().display());
        println!("cargo:rustc-link-lib=static=cef_sandbox");

        // Link dynamically to CEF.
        println!("cargo:rustc-link-lib=dylib=libcef");

        // Link dynamically to CEF dependencies.
        println!("cargo:rustc-link-lib=wbemuuid");
        println!("cargo:rustc-link-lib=propsys");
        println!("cargo:rustc-link-lib=delayimp");
    }

    Ok(())
}
