//! Build script embedding `assets/app.ico` as Win32 Resource ID `1` so GPUI's
//! Windows platform backend (`LoadImageW(..., PCWSTR(1), IMAGE_ICON, ...)`)
//! automatically displays the app icon on the taskbar, Alt-Tab switcher,
//! Explorer executable, and native window titlebar.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=assets/app.ico");

    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }

    let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") else {
        return;
    };
    let Ok(out_dir) = env::var("OUT_DIR") else {
        return;
    };

    let ico_path = PathBuf::from(&manifest_dir).join("assets").join("app.ico");
    if !ico_path.exists() {
        return;
    }

    let Some(rc_exe) = find_rc_exe() else {
        println!("cargo:warning=Windows SDK rc.exe not found; skipping app.ico resource embedding");
        return;
    };

    let rc_path = PathBuf::from(&out_dir).join("app_icon.rc");
    let res_path = PathBuf::from(&out_dir).join("app_icon.res");
    let escaped_ico = ico_path.to_string_lossy().replace('\\', "\\\\");
    let rc_content = format!("1 ICON \"{escaped_ico}\"\n");

    if fs::write(&rc_path, rc_content).is_err() {
        return;
    }

    let status = Command::new(&rc_exe)
        .arg("/nologo")
        .arg("/fo")
        .arg(&res_path)
        .arg(&rc_path)
        .status();

    if let Ok(exit) = status
        && exit.success()
        && res_path.exists()
    {
        println!("cargo:rustc-link-arg-bins={}", res_path.display());
    }
}

fn find_rc_exe() -> Option<PathBuf> {
    if let Ok(status) = Command::new("rc.exe").arg("/?").output()
        && status.status.success()
    {
        return Some(PathBuf::from("rc.exe"));
    }

    let kits_root = Path::new(r"C:\Program Files (x86)\Windows Kits\10\bin");
    let entries = fs::read_dir(kits_root).ok()?;
    let mut versions: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .collect();
    versions.sort();
    versions.reverse();

    for ver_dir in versions {
        let candidate = ver_dir.join("x64").join("rc.exe");
        if candidate.exists() {
            return Some(candidate);
        }
    }
    None
}
