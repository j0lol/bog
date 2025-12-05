use std::{env, error::Error, fs::copy, path::PathBuf, process::Command};

use vergen_git2::{BuildBuilder, CargoBuilder, Emitter, Git2Builder, RustcBuilder, SysinfoBuilder};

pub fn main() -> Result<(), Box<dyn Error>> {
    vergen()?;
    bundle_js()?;

    Ok(())
}

pub fn bundle_js() -> Result<(), Box<dyn Error>> {
    println!("cargo::rerun-if-changed=src/index.js");
    println!("cargo::rerun-if-changed=src/login.js");
    println!("cargo::rerun-if-changed=src/bun.lock");

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let out_dir = out_dir.to_str().ok_or("malformed osstr")?;

    let status = Command::new("bun")
        .args([
            "build",
            &format!("--outfile={out_dir}/bundle.js"),
            "src/index.js",
        ])
        .status()?;
    assert!(status.success());

    let _ = std::fs::remove_file(format!("{out_dir}/modern-normalize.css"));
    copy(
        "node_modules/modern-normalize/modern-normalize.css",
        format!("{out_dir}/modern-normalize.css"),
    )?;

    // println!("cargo:rerun-if-changed=bundle.js");

    Ok(())
}

// NOTE: This will output everything, and requires all features enabled.
// NOTE: See the specific builder documentation for configuration options.
pub fn vergen() -> Result<(), Box<dyn Error>> {
    let build = BuildBuilder::all_build()?;
    let cargo = CargoBuilder::all_cargo()?;
    let git2 = Git2Builder::all_git()?;
    let rustc = RustcBuilder::all_rustc()?;
    let si = SysinfoBuilder::all_sysinfo()?;

    Emitter::default()
        .add_instructions(&build)?
        .add_instructions(&cargo)?
        .add_instructions(&git2)?
        .add_instructions(&rustc)?
        .add_instructions(&si)?
        .emit()?;

    Ok(())
}
