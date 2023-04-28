use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use anyhow::{anyhow, bail};
use lazy_static::lazy_static;
use regex::Regex;

use crate::app::AppResult;

struct Env {
    credentials_file: PathBuf,
}

impl Env {
    fn read() -> AppResult<Self> {
        let credentials_file: PathBuf = dirs::home_dir()
            .ok_or(anyhow!("Failed to get home directory"))
            .map(|p| p.join(".aws/credentials"))?;

        Ok(Env { credentials_file })
    }
}

struct Parser;

impl Parser {
    fn parse(credentials_file: PathBuf) -> AppResult<Vec<String>> {
        let credentials = match File::open(credentials_file.as_path()) {
            Ok(f) => f,
            Err(err) => bail!("Error: {}", err),
        };

        let reader = BufReader::new(credentials);
        let mut names: Vec<String> = vec![];

        lazy_static! {
            static ref PROFILE_NAME: Regex = Regex::new(r"\[(.*?)\]").unwrap();
        }

        for line in reader.lines() {
            let content = line.unwrap();
            if content.starts_with("[") {
                for cap in PROFILE_NAME.captures_iter(content.as_str()) {
                    names.push(String::from(&cap[1]));
                }
            }
        }

        Ok(names)
    }
}

#[derive(Clone, Debug)]
pub struct Profile {
    pub name: String,
}

#[derive(Debug)]
pub struct ProfileSet {
    pub profiles: Vec<Profile>,
}

impl ProfileSet {
    pub fn load() -> AppResult<Self> {
        let env = Env::read()?;
        if !env.credentials_file.exists() {
            bail!(
                "Currently requires credentials file to exist at {}",
                env.credentials_file.display()
            )
        }

        let names = Parser::parse(env.credentials_file)?;
        let profiles: Vec<Profile> = names
            .iter()
            .map(|name| Profile {
                name: name.to_owned(),
            })
            .collect();

        Ok(ProfileSet { profiles })
    }
}
