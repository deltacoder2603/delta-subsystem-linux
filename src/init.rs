use std::env;
use std::ffi::CString;
use std::fs;

use nix::errno::Errno;
use nix::mount::{mount, MsFlags};
use nix::unistd::execvp;

fn mount_ignore_busy(
    source: Option<&str>,
    target: &str,
    fstype: Option<&str>,
    flags: MsFlags,
) {
    match mount(source, target, fstype, flags, None::<&str>) {
        Ok(_) => {}
        Err(e) if e == Errno::EBUSY => {
        }
        Err(e) => {
            panic!("failed to mount {}: {}", target, e);
        }
    }
}

pub fn start() {
    fs::create_dir_all("/proc").ok();
    fs::create_dir_all("/sys").ok();
    fs::create_dir_all("/dev").ok();

    mount_ignore_busy(Some("proc"), "/proc", Some("proc"), MsFlags::empty());
    mount_ignore_busy(Some("sysfs"), "/sys", Some("sysfs"), MsFlags::empty());
    mount_ignore_busy(Some("/dev"), "/dev", None, MsFlags::MS_BIND);

    unsafe {
        env::set_var("PS1", "dsl \\u@\\h:\\w\\$ ");
        env::set_var("TERM", "xterm-256color");
        env::set_var("HOME", "/root");
        env::set_var(
            "PATH",
            "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
        );
    }

    let shell = CString::new("/bin/busybox").unwrap();
    let arg = CString::new("sh").unwrap();

    execvp(&shell, &[shell.clone(), arg]).unwrap();
}
