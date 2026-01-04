use nix::unistd::{fork, ForkResult};
use crate::{namespace, filesystem, cgroup, init};
use crate::cli::Limits;

pub fn run(distro: &str, limits: Limits) {
    namespace::setup();

    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            cgroup::apply(&limits);

            filesystem::setup(distro);

            init::start();
        }
        Ok(ForkResult::Parent { .. }) => {
            let _ = nix::sys::wait::wait();
        }
        Err(_) => panic!("fork failed"),
    }
}

