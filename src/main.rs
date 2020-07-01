extern crate termion;

use std::process;
use std::env;

mod config;
mod interactive;
mod kubectl;
mod normal;

fn main() {
    let config = config::Opt::new_from_args(env::args());
    if config.is_err() {
        eprintln!(r#"usage: klo <context> <namespace>
           <context | namespace-in-current-context>
           (no arguments for interactive mode)"#);
        process::exit(1);
    }

    let config = config.and_then(|c| {
        match c.mode {
            config::Mode::Normal => normal::run(&c),
            config::Mode::Interactive => interactive::run(),
        }
    });

    if let Err(e) = config {
        eprintln!("{}", e);
        process::exit(1);
    }
}
