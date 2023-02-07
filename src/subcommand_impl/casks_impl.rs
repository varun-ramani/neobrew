use clap::ArgMatches;
use crate::subcommand_impl::err_not_impl;

pub fn run(matches: &ArgMatches) -> Result<(), String> {
    err_not_impl();
    Err(String::from("Not implemented"))
}