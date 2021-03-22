#[cfg(target_os = "windows")]
fn main() {
    use std::process::Command;
    use std::path::Path;

    let rustup_output = Command::new("rustup")
        .arg("which")
        .arg("rustc")
        .output()
        .expect("Couldn't get rustup output.");
    let rustc_path = String::from_utf8(rustup_output.stdout).expect("Couldn't get toolchain path");
    let toolchain_path = Path::new(&rustc_path)
        .parent().unwrap()
        .parent().unwrap();

    let toolchain_triple = toolchain_path
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .map(|name| name.replace("stable-", ""))
        .expect("Couldn't get toolchain triple.");
    let architecture = if let Some(_) = toolchain_triple.find("x86_64") {
        "x86_64"
    } else {
        "x86"
    };

    let source_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("redist").join(architecture);
    let dll_path = source_path.join("gmp.dll");
    let lib_path = source_path.join("gmp.lib");
    let target_path = toolchain_path
        .join("lib")
        .join("rustlib")
        .join(toolchain_triple)
        .join("lib");
    std::fs::copy(dll_path, target_path.join("gmp.dll")).expect("Couldn't copy dll");
    std::fs::copy(lib_path, target_path.join("gmp.lib")).expect("Couldn't copy lib");
}

#[cfg(not(target_os = "windows"))]
fn main() {}