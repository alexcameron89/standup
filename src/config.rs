static DEFAULT_DIRECTORY: &'static str = "standups";
static DEFAULT_STANDUP_TEMPLATE_LOC: &'static str = ".standup_template";

use clap::{App, ArgMatches, SubCommand};
use standup::StandupCommand;
use std::path::{Path, PathBuf};

pub struct Config<'a> {
    pub directory: PathBuf,
    pub matches: ArgMatches<'a>,
    pub standup_template: PathBuf,
}

pub fn build<'a>() -> Config<'a> {
    let matches = arg_matches();
    let home = dirs::home_dir().unwrap();
    let directory = Path::new(&home).join(DEFAULT_DIRECTORY);
    let standup_template = Path::new(&home).join(DEFAULT_STANDUP_TEMPLATE_LOC);
    Config {
        matches,
        directory,
        standup_template,
    }
}

impl<'a> Config<'a> {
    pub fn command(&self) -> StandupCommand {
        match self.matches.subcommand_name() {
            Some("init") => StandupCommand::Initialize,
            _ => StandupCommand::Run,
        }
    }
}

fn arg_matches<'a>() -> ArgMatches<'a> {
    App::new("standup")
        .version("0.1")
        .about("Logs standups for later viewing")
        .author("Alex Kitchens")
        .subcommand(SubCommand::with_name("init").about("initiates the standup folder"))
        .get_matches()
}
