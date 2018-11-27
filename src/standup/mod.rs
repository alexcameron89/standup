use chrono::prelude::*;
use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;

use config::Config;

pub type InitResult = Result<String, String>;
pub type StandupResult = Result<String, String>;

static DEFAULT_STANDUP_TEMPLATE: &'static str = "## Previous\n## Today\n";

pub enum StandupCommand {
    Initialize,
    Run,
}

pub fn initiate_directory(config: Config) -> InitResult {
    let path = config.directory;

    if path.exists() {
        Err(format!("{} already exists", path.display()))
    } else {
        fs::create_dir(&path).unwrap();
        Ok(format!("{} was successfully created", path.display()))
    }
}

/* This method should:
 * 1. See if the standup directory exists and error if it doesn't
 * 2. See if today's standup already exists
 * 3. Get the previous standup or standup template if it does not
 * 4. Open Vim
 *
 */
pub fn initiate(config: Config) -> StandupResult {
    match assert_standup_directory_exists(&config) {
        Ok(_) => {},
        Err(_) => {
            return Err("Standup has not been initiated and the directory does not exist.\nYou can initiate Standup with the following:\n\tstandup init".to_string());
        }
    }

    let date_string = Local::today().format("%Y-%m-%d").to_string();
    let standup_filename = filename(&config, date_string);
    let content = get_standup_template(config, &standup_filename);

    Command::new("vim")
        .arg("-c")
        .arg("set nu!")
        .arg("-c")
        .arg(&(format!("normal i{}", content)))
        .arg(standup_filename.to_str().unwrap())
        .status()
        .expect("sh command failed to start");

    Ok("Standup was successful".to_string())
}

fn assert_standup_directory_exists(config: &Config) -> Result<(), ()> {
    match config.directory.exists() {
        true => Ok(()),
        false => Err(()),
    }
}

fn get_standup_template(config: Config, filename: &PathBuf) -> String {
    if filename.exists() {
        String::from("")
    } else {
        read_previous_standup(config)
    }
}

fn read_previous_standup(config: Config) -> String {
    let paths = fs::read_dir(&config.directory).expect("The standup directory was not found");

    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    let previous_standup_filename = paths
        .into_iter()
        .map(|path| {
            path.unwrap()
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        })
        .filter(|f| re.is_match(f))
        .max();

    let old_filename = match previous_standup_filename {
        Some(date_filename) => filename(&config, date_filename),
        None => config.standup_template,
    };

    match File::open(old_filename) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("something went wrong reading the the old standup");

            contents
        },
        _ => { DEFAULT_STANDUP_TEMPLATE.to_string() }
    }
}

fn filename(config: &Config, date_string: String) -> PathBuf {
    let mut standup_filename = config.directory.clone();
    standup_filename.push(format!("{}.md", date_string));

    standup_filename.to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;
    use config::Config;
    use clap::App;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_standup_w_directory_wo_prev_standup_w_template() {
        let matches = App::new("Test").get_matches();
        let directory = Path::new("./tests/test_empty_standup_directory").to_path_buf();
        let standup_template = Path::new("./tests/test_standup_template.md").to_path_buf();
        let _ = fs::create_dir(&directory);
        let config = Config {
            matches,
            directory,
            standup_template,
        };

        let template =
            get_standup_template(config, &Path::new("random_nonexistent_file").to_path_buf());
        assert_eq!(template, "## Yesterday\n## Today\n");
    }

    #[test]
    fn test_standup_w_directory_wo_prev_standup_wo_template() {
        let matches = App::new("Test").get_matches();
        let directory = Path::new("./tests/test_empty_standup_directory").to_path_buf();
        let standup_template = Path::new("./tests/nonexistent_standup_template.md").to_path_buf();
        let _ = fs::create_dir(&directory);
        let config = Config {
            matches,
            directory,
            standup_template,
        };

        let template =
            get_standup_template(config, &Path::new("random_nonexistent_file").to_path_buf());
        assert_eq!(template, "## Previous\n## Today\n");
    }

    #[test]
    fn test_standup_w_directory_w_prev_standup() {
        let matches = App::new("Test").get_matches();
        let directory = Path::new("./tests/test_standup_directory").to_path_buf();
        let standup_template = Path::new("./tests/test_standup_template.md").to_path_buf();
        let previous_standup = "This is the previous standup";
        File::create(&Path::new("./tests/test_standup_directory/2011-11-11.md"))
            .unwrap()
            .write_all(previous_standup.as_bytes())
            .unwrap();
        let config = Config {
            matches,
            directory,
            standup_template,
        };

        let template =
            get_standup_template(config, &Path::new("random_nonexistent_file").to_path_buf());

        let _ = fs::remove_file("./tests/test_standup_directory/2011-11-11.md");

        assert_eq!("This is the previous standup", template);
    }

    #[test]
    fn test_standup_w_directory_w_current_standup() {
        let matches = App::new("Test").get_matches();
        let directory = Path::new("./tests/test_standup_directory").to_path_buf();
        let standup_template = Path::new("./tests/test_standup_template.md").to_path_buf();
        let config = Config {
            matches,
            directory,
            standup_template,
        };
        let current_standup = "This is the current standup";
        File::create(&Path::new("./tests/test_standup_directory/2010-01-12.md"))
            .unwrap()
            .write_all(current_standup.as_bytes())
            .unwrap();

        let template = get_standup_template(
            config,
            &Path::new("./tests/test_standup_directory/2010-01-12.md").to_path_buf(),
        );
        let _ = fs::remove_file("./tests/test_standup_directory/2010-01-12.md");

        assert_eq!(template, "");
    }
}
