pub mod cli;

use cli::Args;
use log::{debug, info, warn};
use std::{fs, process::Command, time::Instant};

pub struct Execution {
    directory: String,
    legacy: bool,
    pub yalc: bool,
}

impl Execution {
    /// Creates a new `Execution` instance based on the provided command-line arguments (`Args`).
    ///
    /// # Arguments
    ///
    /// * `args` - The command-line arguments (`Args`) to initialize the `Execution`.
    ///
    /// # Returns
    ///
    /// A new `Execution` instance.
    pub fn new(args: &Args) -> Self {
        info!("Initializing ruset with the params: {:?}\n", &args);

        Self {
            directory: get_dir(args),
            yalc: args.yalc,
            legacy: args.legacy,
        }
    }

    /// Removes the Yalc entries in the specified directory.
    ///
    /// # Returns
    ///
    /// An `Ok` result if the Yalc installation was successfully removed, or an `Err` containing an error message.
    pub fn remove_yalc_installation(&self) -> Result<(), String> {
        let time_removing_yalc = Instant::now();
        let yalc_folder = format!("{}/.yalc", self.directory);
        let yalc_file = format!("{}/yalc.lock", self.directory);

        warn!("Yalc arg detected");
        info!("Removing yalc");
        info!(
            "Yalc entries: .yalc {} - yalc.lock {}",
            &yalc_folder, &yalc_file
        );

        if let Err(error) = fs::remove_file(yalc_file) {
            match error.kind() {
                _ => return Err(error.to_string()),
            }
        }

        if let Err(error) = fs::remove_dir_all(yalc_folder) {
            match error.kind() {
                _ => return Err(error.to_string()),
            }
        }

        debug!(
            "[remove_yalc_installation]: {:2.?}\n",
            time_removing_yalc.elapsed()
        );

        Ok(())
    }

    /// Removes the `node_modules` directory from the specified directory.
    ///
    /// # Returns
    ///
    /// An `Ok` result if the `node_modules` directory was successfully removed, or an `Err` containing an error message.
    pub fn remove_node_modules(&self) -> Result<(), String> {
        let time_removing_node_modules = Instant::now();
        let node_modules_folder = format!("{}/node_modules", self.directory);

        info!("Removing node_modules");
        info!("Directory {}", &node_modules_folder);

        if let Err(error) = fs::remove_dir_all(node_modules_folder) {
            match error.kind() {
                _ => return Err(error.to_string()),
            }
        }

        debug!(
            "[remove_node_modules]: {:2.?}\n",
            time_removing_node_modules.elapsed()
        );

        Ok(())
    }

    /// Installs `npm` packages in the specified directory.
    ///
    /// # Returns
    ///
    /// An `Ok` result if the `npm` dependencies were successfully installed, or an `Err` containing an error message.
    pub fn install_npm_dependencies(&self) -> Result<(), String> {
        let time_installing_npm_dependencies = Instant::now();

        info!("Installing dependencies");

        let mut npm = Command::new("npm");

        npm.args(["--prefix", &self.directory, "install"]);

        if self.legacy {
            warn!("Legacy arg detected, installing using --legacy-peer-deps");
            npm.arg("--legacy-peer-deps");
        }

        if let Err(error) = npm.status() {
            return Err(error.to_string());
        }

        println!();
        debug!(
            "[install_npm_dependencies]: {:2.?}\n",
            time_installing_npm_dependencies.elapsed()
        );

        Ok(())
    }
}

/// Returns the directory path specified in the command-line arguments (`Args`) or a default path if not provided.
///
/// # Arguments
///
/// * `args` - The command-line arguments (`Args`) containing the directory path.
///
/// # Returns
///
/// The directory path as a `String`.
fn get_dir(args: &Args) -> String {
    let default_directory = String::from(".");

    let directory = match &args.dir {
        Some(dir) => dir,
        None => &default_directory,
    };

    return directory.to_owned();
}
