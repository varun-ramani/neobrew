use clap::{Command, Arg, arg};

pub fn casks_cmd() -> Command {
    Command::new("List Casks")
        .name("casks")
}

pub fn install_cmd() -> Command {
    Command::new("Install")
        .name("install")
        .about("install a formula or a cask")
        .arg(
            arg!(--cask "try installing provided packages as a cask")
        )
        .arg(
            Arg::new("packages")
                .num_args(1..)
                .required(true)
        )
}

