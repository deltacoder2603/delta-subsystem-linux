use std::env;
use std::path::{Path, PathBuf};

use nix::unistd::{chdir, chroot};

fn find_project_root() -> PathBuf {
    let cwd = env::current_dir().expect("failed to get current directory");

    if cwd.join("distros").is_dir() {
        return cwd;
    }

    if let Some(parent) = cwd.parent() {
        if parent.join("distros").is_dir() {
            return parent.to_path_buf();
        }
    }

    panic!(
        "Could not find 'distros/' directory.\n\
         Run DSL from the project root (the directory that contains 'distros').\n\
         Current directory: {:?}",
        cwd
    );
}

pub fn setup(distro: &str) {
    let project_root = find_project_root();

    let rootfs = project_root
        .join("distros")
        .join(distro)
        .join("rootfs");

    if !rootfs.is_dir() {
        panic!(
            "rootfs not found for distro '{}'\n\
             Expected path: {:?}",
            distro, rootfs
        );
    }

    chroot(&rootfs).expect("chroot failed");

    chdir("/").expect("chdir to / failed");
}


