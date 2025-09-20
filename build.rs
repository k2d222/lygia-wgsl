use std::path::Path;
use std::process::Command;

fn fetch_repo(url: &str, rev: &str, dest: &Path) {
    // Modeled after https://github.com/gfx-rs/wgpu/blob/c0a580d6f0343a725b3defa8be4fdf0a9691eaad/xtask/src/cts.rs
    if std::fs::exists(dest).unwrap() {
        // Do a git update
        let commit_exists = Command::new("git")
            .args(["cat-file", "commit", rev])
            .current_dir(dest)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("failed to execute git cat-file")
            .wait()
            .expect("failed to wait on git")
            .success();

        if !commit_exists {
            let git_fetch = Command::new("git")
                .args(["fetch", "--quiet"])
                .current_dir(dest)
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::inherit())
                .spawn()
                .expect("failed to execute git fetch")
                .wait()
                .expect("failed to wait on git");
            if !git_fetch.success() {
                panic!("Git fetch failed");
            }
        }

        let git_checkout = Command::new("git")
            .args(["checkout", "--quiet", rev])
            .current_dir(dest)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::inherit())
            .spawn()
            .expect("failed to execute git checkout")
            .wait()
            .expect("failed to wait on git");

        if !git_checkout.success() {
            panic!("Git checkout failed");
        }
    } else {
        let git_clone = Command::new("git")
            .args([
                "clone",
                "--depth=1",
                url,
                "--revision",
                rev,
                dest.to_str().unwrap(),
            ])
            .current_dir(dest.parent().unwrap())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::inherit())
            .spawn()
            .expect("failed to execute git clone")
            .wait()
            .expect("failed to wait on git");

        if !git_clone.success() {
            panic!("Git clone failed");
        }
    }
}

fn main() {
    let lygia_repo = "https://github.com/k2d222/wesl-lygia";
    let lygia_rev = "774f83f6e890e81003cf41cbedc8a3fa1daf00b5";
    let lygia_path = Path::new("./lygia");

    fetch_repo(lygia_repo, lygia_rev, lygia_path);

    // currently lygia contains wgsl files which interfere with wesl build system.
    // Command::new("git")
    //     .args(["rm", "'*.wgsl'"])
    //     .current_dir(lygia_path)
    //     .stdout(std::process::Stdio::null())
    //     .stderr(std::process::Stdio::inherit())
    //     .spawn()
    //     .expect("failed to execute git clone")
    //     .wait()
    //     .expect("failed to wait on git");

    wesl::PkgBuilder::new("lygia")
        .scan_root("lygia")
        .expect("failed to scan WESL files")
        .validate()
        .map_err(|e| eprintln!("{e}"))
        .expect("validation error")
        .build_artifact()
        .expect("failed to build artifact");
}
