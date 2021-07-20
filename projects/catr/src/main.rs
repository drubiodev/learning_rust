use catr::{get_args, run};
use std::process;

fn main() {
    let config = match get_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    if let Err(e) = run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
