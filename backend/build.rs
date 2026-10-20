use flate2::read::GzDecoder;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::Path;
use tar::Archive;
use tempfile::TempDir;

use pdfium_render::prelude::Pdfium;

const PDFIUM_VERSION: &str = "chromium%2F7763";
const URL_BASE: &str = "https://github.com/bblanchon/pdfium-binaries/releases/download";

struct BuildConfig {
    lib_dir: Box<Path>,
    lib_file_name: OsString,
}

#[derive(Debug)]
enum BuildError {
    IoError(io::Error),
    ReqwestError(reqwest::Error),
    EnvVarError(env::VarError),
    VersionFileNotFound,
    UnsupportedPlatform,
}

impl From<io::Error> for BuildError {
    fn from(error: io::Error) -> Self {
        BuildError::IoError(error)
    }
}

impl From<reqwest::Error> for BuildError {
    fn from(error: reqwest::Error) -> Self {
        BuildError::ReqwestError(error)
    }
}

impl From<env::VarError> for BuildError {
    fn from(error: env::VarError) -> Self {
        BuildError::EnvVarError(error)
    }
}

fn get_build_target() -> Option<&'static str> {
    let os = std::env::consts::OS;
    let arch = if cfg!(target_arch = "arm") {
        "arm"
    } else if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "x86") {
        "x86"
    } else {
        "unsupported"
    };

    match (os, arch) {
        ("linux", "arm") => Some("linux-arm"),
        ("linux", "aarch64") => Some("linux-arm64"),
        ("linux", "x86_64") => Some("linux-x64"),
        ("linux", "x86") => Some("linux-x86"),
        ("macos", "aarch64") => Some("mac-arm64"),
        ("macos", "x86_64") => Some("mac-x64"),
        ("macos", _) => Some("mac-univ"),
        ("windows", "aarch64") => Some("win-arm64"),
        ("windows", "x86_64") => Some("win-x64"),
        ("windows", "x86") => Some("win-x86"),
        _ => None,
    }
}

fn inject_version() -> Result<(), BuildError> {
    let content = fs::read_to_string("../VERSION").map_err(|_| BuildError::VersionFileNotFound)?;

    // Parse version and codename from the file
    let mut version = String::new();
    let mut codename = String::new();

    for line in content.lines() {
        if let Some(v) = line.strip_prefix("version=") {
            version = v.trim().to_string();
        } else if let Some(c) = line.strip_prefix("codename=") {
            codename = c.trim().to_string();
        }
    }

    if version.is_empty() || codename.is_empty() {
        return Err(BuildError::VersionFileNotFound);
    }

    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("version.rs");

    let version_content = format!(
        r#"
           pub const VERSION: &str = "{}";
           pub const CODENAME: &str = "{}";
        "#,
        version, codename
    );

    fs::write(dest_path, version_content)?;

    println!("cargo:warning=Injecting version {} ({})", version, codename);

    Ok(())
}

fn download_pdfium(config: &BuildConfig, target: &str) -> Result<(), BuildError> {
    let temp_dir = TempDir::new()?;
    let target_file = format!("pdfium-{}.tgz", target);
    let download_url = format!("{}/{}/{}", URL_BASE, PDFIUM_VERSION, target_file);

    // Download the tgz file
    let mut response = reqwest::blocking::get(&download_url)?;
    let downloaded_file_path = temp_dir.path().join(&target_file);
    let mut downloaded_file = fs::File::create(&downloaded_file_path)?;
    response.copy_to(&mut downloaded_file)?;

    // Extract the archive
    let file = fs::File::open(&downloaded_file_path)?;
    let tar = GzDecoder::new(file);
    let mut archive = Archive::new(tar);
    archive.unpack(&temp_dir.path())?;

    // Move files to final location
    let bin_dir = if target.contains("win") { "bin" } else { "lib" };
    let bin_file = temp_dir.path().join(bin_dir).join(&config.lib_file_name);
    let version_file = temp_dir.path().join("VERSION");

    fs::create_dir_all(&config.lib_dir)?;

    let lib_file = config.lib_dir.join(&config.lib_file_name);
    fs::rename(&bin_file, &lib_file)?;
    fs::rename(&version_file, config.lib_dir.join("VERSION"))?;

    Ok(())
}

fn setup_pdfium() -> Result<(), BuildError> {
    let config = BuildConfig {
        lib_dir: Path::new("./lib").into(),
        lib_file_name: Pdfium::pdfium_platform_library_name(),
    };

    let lib_file = config.lib_dir.join(&config.lib_file_name);

    if !lib_file.exists() {
        if let Some(target) = get_build_target() {
            download_pdfium(&config, target)?;
        } else {
            return Err(BuildError::UnsupportedPlatform);
        }
    }

    Ok(())
}

fn main() {
    // Inject version information
    if let Err(e) = inject_version() {
        eprintln!("Failed to inject version: {:?}", e);
        std::process::exit(1);
    }

    // Setup PDFium
    if let Err(e) = setup_pdfium() {
        eprintln!("Failed to setup PDFium: {:?}", e);
        std::process::exit(1);
    }
}
