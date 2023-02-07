pub mod casks_impl;
pub mod install_impl;
use colored::Colorize;

pub fn err_not_impl() {
    println!(
        "{} - {}",
        "NOT IMPLEMENTED YET".bold().red(),
        "But I'll get around to it soon :)".green()
    );
}