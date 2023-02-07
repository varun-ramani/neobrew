use clap::{ArgMatches};
use colored::Colorize;

pub fn install_casks(cask_names: &Vec<&String>) {
    
}

pub fn run(matches: &ArgMatches) -> Result<(), String> {
    if matches.get_flag("cask") {
        println!("{}", "Installing as Cask(s)".bold().cyan());
        let cask_names: Vec<&String> = matches
            .get_many::<String>("packages")
            .ok_or(String::from("Failed to obtain packages from cli"))?
            .into_iter()
            .collect();
        install_casks(&cask_names);
    }       

    Ok(())
}