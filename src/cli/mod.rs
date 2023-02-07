use clap::Command;
mod subcommand;

pub fn clap_cmd() -> clap::Command {
    Command::new("NeoBrew")
        .about("A Rust rewrite of Homebrew's core functionality")
        .name("neobrew")
        .subcommand(subcommand::casks_cmd())
        .subcommand(subcommand::install_cmd())
        .arg_required_else_help(true)
}