use log::{error, info};
use ruset::{cli::get_args, Execution};
use simple_logger::SimpleLogger;
use std::{process::ExitCode, time::Instant};

fn main() -> ExitCode {
    SimpleLogger::new().init().unwrap();

    let time_global = Instant::now();

    let args = get_args();
    let exe = Execution::new(&args);

    if !args.install_only {
        if exe.yalc {
            if let Err(error) = exe.remove_yalc_installation() {
                error!("{}", error);
                return ExitCode::FAILURE;
            }
        }

        if let Err(error) = exe.remove_node_modules() {
            error!("{}", error);
            return ExitCode::FAILURE;
        }
    }

    if let Err(error) = exe.install_npm_dependencies() {
        error!("{}", error);
        return ExitCode::FAILURE;
    }

    info!("Time spend {:.2?}", time_global.elapsed());

    ExitCode::SUCCESS
}
