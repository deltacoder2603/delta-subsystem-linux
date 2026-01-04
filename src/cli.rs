use std::env;
use crate::runtime;

#[derive(Debug, Clone)]
pub struct Limits {
    pub memory: Option<String>,
    pub cpu: Option<u64>,
    pub pids: Option<u64>,
}

fn print_header() {
    println!();
    println!("██████╗ ███████╗██╗     ");
    println!("██╔══██╗██╔════╝██║     ");
    println!("██║  ██║███████╗██║     ");
    println!("██║  ██║╚════██║██║     ");
    println!("██████╔╝███████║███████╗");
    println!("╚═════╝ ╚══════╝╚══════╝");
    println!();
    println!("Delta Subsystem for Linux");
    println!("Minimal. Isolated. Fast.");
    println!("────────────────────────────────────────");
    println!();
}

fn print_usage() {
    println!("Usage");
    println!("─────");
    println!("  dsl run <distro> [options]");
    println!();
    println!("Options");
    println!("───────");
    println!("  --memory <size>   Memory limit (e.g. 256M, 1G)");
    println!("  --cpu <n>         CPU quota (cores)");
    println!("  --pids <n>        Max process count");
    println!();
    println!("Example");
    println!("───────");
    println!("  dsl run alpine --memory 512M --cpu 1 --pids 64");
    println!();
}

fn print_config(distro: &str, limits: &Limits) {
    println!("Runtime Configuration");
    println!("─────────────────────");
    println!("Distro   : {}", distro);

    match &limits.memory {
        Some(v) => println!("Memory   : {}", v),
        None => println!("Memory   : unlimited"),
    }

    match limits.cpu {
        Some(v) => println!("CPU      : {}", v),
        None => println!("CPU      : unlimited"),
    }

    match limits.pids {
        Some(v) => println!("PIDs     : {}", v),
        None => println!("PIDs     : unlimited"),
    }

    println!();
    println!("Launching isolated environment...");
    println!();
}

pub fn handle() {
    let args: Vec<String> = env::args().collect();

    print_header();

    if args.len() < 3 || args[1] != "run" {
        print_usage();
        return;
    }

    let distro = args[2].clone();

    let mut limits = Limits {
        memory: None,
        cpu: None,
        pids: None,
    };

    let mut i = 3;
    while i < args.len() {
        match args[i].as_str() {
            "--memory" => {
                if i + 1 < args.len() {
                    limits.memory = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--cpu" => {
                if i + 1 < args.len() {
                    limits.cpu = Some(
                        args[i + 1]
                            .parse()
                            .expect("invalid --cpu value"),
                    );
                    i += 1;
                }
            }
            "--pids" => {
                if i + 1 < args.len() {
                    limits.pids = Some(
                        args[i + 1]
                            .parse()
                            .expect("invalid --pids value"),
                    );
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    print_config(&distro, &limits);

    runtime::run(&distro, limits);
}
