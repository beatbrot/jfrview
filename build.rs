use std::{
    env::{self, VarError},
    error::Error,
    fs::File,
    io::Write,
    path::PathBuf,
};

use image::{EncodableLayout, ImageReader};

const IMAGE_FILENAME: &str = "icon";

fn main() -> Result<(), Box<dyn Error>> {
    convert_icon()?;
    Ok(())
}

/// Converts "icon.png" in the repo root to "icon.rgba"
fn convert_icon() -> Result<(), Box<dyn Error>> {
    let screenshot_path = project_dir()?.join(format!("{IMAGE_FILENAME}.png"));
    rerun_if_changed(&screenshot_path);

    let img = ImageReader::open(screenshot_path)?.decode()?.into_rgba8();

    let out_path = project_dir()?.join(format!("{IMAGE_FILENAME}.rgba"));
    rerun_if_changed(&out_path);
    let mut out_file = File::create(out_path)?;
    out_file.write_all(img.as_bytes())?;
    Ok(())
}

/// Returns the directory of this project
fn project_dir() -> Result<PathBuf, VarError> {
    env::var("CARGO_MANIFEST_DIR").map(|e| PathBuf::from(e))
}

/// The build script will be re-ran if this path changes
fn rerun_if_changed(path: &PathBuf) {
    println!("cargo::rerun-if-changed={}", path.to_string_lossy());
}
