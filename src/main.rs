use std::{fs, io::ErrorKind, process::Command, time::Instant};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory to reset
    #[arg(short, long)]
    dir: String,

    /// Install npm dependencies as legacy
    #[arg(short, long)]
    legacy: bool,
}

fn remove_node_modules(args: &Args) {
    println!("Removing node_modules...\n");

    let node_modules_folder = format!("{}/node_modules", &args.dir);

    fs::remove_dir_all(node_modules_folder).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            panic!("The directory was not found {}", &args.dir);
        } else {
            panic!("{:?}", error);
        }
    });

    println!("Done!\n");
}

fn install_npm_dependencies(args: &Args) {
    println!("Installing npm packages...");

    let mut npm = Command::new("npm");

    npm.args(["--prefix", &args.dir, "install"]);

    if args.legacy {
        println!("Legacy flag detected, installing using --legacy-peer-deps");

        npm.arg("--legacy-peer-deps");
    }

    println!("\n");

    npm.status().expect("Algo?");
}

fn main() {
    let now = Instant::now();

    let args = Args::parse();

    remove_node_modules(&args);

    install_npm_dependencies(&args);

    println!("\n\nTime spend {:.2?}", now.elapsed())
}
