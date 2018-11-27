extern crate chrono;
extern crate clap;
extern crate dirs;
extern crate regex;

mod config;
mod standup;

use standup::StandupCommand;

fn main() {
    let config = config::build();

    match config.command() {
        StandupCommand::Initialize => match standup::initiate_directory(config) {
            Ok(message) => {
                println!("{}", message);
            }
            Err(message) => {
                println!("{}", message);
            }
        },
        StandupCommand::Run => match standup::initiate(config) {
            Ok(message) => {
                println!("{}", message);
            }
            Err(message) => {
                println!("{}", message);
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use config::Config;
    use clap::App;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_directory_already_exists() {
        let matches = App::new("Test").get_matches();
        let directory = Path::new("/tmp").to_path_buf();
        let standup_template = Path::new("./tests/test_standup_template.md").to_path_buf();
        let config = Config {
            matches,
            directory,
            standup_template,
        };

        let result = standup::initiate_directory(config);

        assert_eq!(
            result,
            Err(String::from("/tmp already exists"))
        );
    }

    #[test]
    fn test_creating_directory() {
        let matches = App::new("Test").get_matches();
        let directory = Path::new("./standup_directory").to_path_buf();
        let standup_template = Path::new("./tests/test_standup_template.md").to_path_buf();
        let config = Config {
            matches,
            directory,
            standup_template,
        };

        let result = standup::initiate_directory(config);
        let _ = fs::remove_dir_all("./standup_directory");

        assert_eq!(
            result,
            Ok(String::from("./standup_directory was successfully created"))
        );
    }

    #[test]
    fn test_standup_without_directory() {
        let matches = App::new("Test").get_matches();
        let directory = Path::new("./standup_directory").to_path_buf();
        let standup_template = Path::new("./tests/test_standup_template.md").to_path_buf();
        let config = Config {
            matches,
            directory,
            standup_template,
        };

        let result = standup::initiate(config);

        assert_eq!(result, Err(String::from("Standup has not been initiated and the directory does not exist.\nYou can initiate Standup with the following:\n\tstandup init")));
    }
}
