use std::fs;
use crate::cli::Limits;

fn parse_memory(mem: &str) -> String {
    let mem = mem.to_uppercase();
    if mem.ends_with("G") {
        let v = mem.trim_end_matches("G").parse::<u64>().unwrap();
        (v * 1024 * 1024 * 1024).to_string()
    } else if mem.ends_with("M") {
        let v = mem.trim_end_matches("M").parse::<u64>().unwrap();
        (v * 1024 * 1024).to_string()
    } else {
        mem
    }
}

pub fn apply(limits: &Limits) {
    let base = "/sys/fs/cgroup/dsl";

    fs::create_dir_all(base).ok();

    if let Some(mem) = &limits.memory {
        fs::write(
            format!("{}/memory.max", base),
            parse_memory(mem),
        ).ok();
    }

    if let Some(cpu) = limits.cpu {
        let quota = cpu * 100_000;
        fs::write(
            format!("{}/cpu.max", base),
            format!("{} 100000", quota),
        ).ok();
    }

    if let Some(pids) = limits.pids {
        fs::write(
            format!("{}/pids.max", base),
            pids.to_string(),
        ).ok();
    }

    fs::write(
        format!("{}/cgroup.procs", base),
        std::process::id().to_string(),
    ).ok();
}
