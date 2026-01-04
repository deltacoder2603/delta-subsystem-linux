mod cli;
mod runtime;
mod namespace;
mod filesystem;
mod cgroup;
mod init;

fn main() {
    cli::handle();
}
