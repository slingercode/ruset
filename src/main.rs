use ruset::{cli::get_args, install_npm_dependencies, remove_node_modules};
use std::time::Instant;

fn main() {
    let time_global = Instant::now();

    let args = get_args();

    remove_node_modules(&args);

    install_npm_dependencies(&args);

    println!("\n\nTime spend {:.2?}", time_global.elapsed());
}
