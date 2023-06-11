pub mod cli;

use cli::Args;
use std::{fs, io::ErrorKind, process::Command, time::Instant};

/// Removes the 'node_modules' directory from the specified directory.
///
/// # Arguments
///
/// * `args` - The command-line arguments (`Args`) containing the directory to reset.
///
/// # Panics
///
/// This function panics if an error occurs while removing the directory or if the directory is not found.
pub fn remove_node_modules(args: &Args) {
    let time_removing_node_modules = Instant::now();
    let node_modules_folder = format!("{}/node_modules", args.dir);

    println!(
        "Removing node_modules from {} directory...\n",
        node_modules_folder
    );

    if let Err(error) = fs::remove_dir_all(node_modules_folder) {
        match error.kind() {
            ErrorKind::NotFound => panic!("The directory was not found {}", args.dir),
            _ => panic!("{:?}", error),
        }
    }

    println!(
        "Done!\n[remove_node_modules]: {:.2?}\n",
        time_removing_node_modules.elapsed()
    );
}

/// Installs npm packages in the specified directory.
///
/// # Arguments
///
/// * `args` - The command-line arguments (`Args`) containing the directory and any installation flags.
///
/// # Panics
///
/// This function panics if an error occurs while executing the npm command.
pub fn install_npm_dependencies(args: &Args) {
    let time_removing_node_modules = Instant::now();

    println!("Installing npm packages...");

    let mut npm = Command::new("npm");

    npm.args(["--prefix", &args.dir, "install"]);

    if args.legacy {
        println!("Legacy flag detected, installing using --legacy-peer-deps");

        npm.arg("--legacy-peer-deps");
    }

    if let Err(error) = npm.status() {
        panic!("Failed to execute npm command: {:?}", error);
    }

    println!(
        "\n\nPackages installed!\n[install_npm_dependencies]: {:.2?}",
        time_removing_node_modules.elapsed()
    );
}
