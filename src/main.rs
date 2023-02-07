mod cli;
mod subcommand_impl;
mod translation;
mod config;

fn main() -> Result<(), String> {
    let cmd = cli::clap_cmd().get_matches();
    match cmd.subcommand() { 
        Some(("casks", matches)) => subcommand_impl::casks_impl::run(matches),
        Some(("install", matches)) => subcommand_impl::install_impl::run(matches),
        _ => Ok(())
    }
}
