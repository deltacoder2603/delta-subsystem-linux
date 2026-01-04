use nix::sched::{unshare, CloneFlags};
use nix::unistd::sethostname;

pub fn setup() {
    unshare(
        CloneFlags::CLONE_NEWPID |
        CloneFlags::CLONE_NEWNS |
        CloneFlags::CLONE_NEWUTS
    ).expect("namespace setup failed");

    sethostname("dsl").unwrap();
}
