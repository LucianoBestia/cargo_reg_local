// region: lmake_readme include "readme.md" //! A
//! # cargo_reg_local
//!
//! Find data from crates.io registry index in local cache.  
//! For now on linux only. The folder of the cache is this:  `~\.cargo\registry\index\github.com-1ecc6299db9ec823\.cache\`  
//!
//! The only argument is a crate name or a substring of the crate name.  
//!
//! The CLI returns:  
//!
//! 1. a list of versions for a given crate_name  
//! 2. all the crate_names (and last version) that contain the given substring  
//!
//! ## Build and run
//!
//! ```bash
//! clear; cargo make dev
//! ```
//!
//! and then use the example how run it in the last 4th line. Like this:
//!
//! ```bash
//! target/debug/cargo_reg_local thread
//! ```

// endregion: lmake_readme include "readme.md" //! A

//region: use statements
use ansi_term::Colour::{Green, Red, Yellow};
use anyhow::{anyhow, Context};
use clap::{App, Arg};
use dirs;
use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

//endregion

/// The CLI program starts here.  
/// Linux only because of file paths. Could be upgraded.  
/// One argument: crate name or a substring of crate name.  
fn main() {
    //define the CLI input line parameters using the clap library
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("crate_name")
                .required(true)
                .help("crate name to search")
                .takes_value(true),
        )
        .get_matches();

    if let Some(crate_name) = matches.value_of("crate_name") {
        //update the local cache, just because we can
        Command::new("cargo")
            .arg("update")
            .arg("--dry-run")
            .status()
            .expect("failed to execute process");

        println!(
            "{} {}",
            Yellow.paint("Crate name or substring:"),
            Yellow.paint(crate_name)
        );

        let versions = get_versions(&crate_name);
        match versions {
            Ok(versions) => {
                println!(
                    "{} '{}' {:#?}",
                    Green.paint("Versions of crate"),
                    Green.paint(crate_name),
                    versions
                );
            }
            Err(err) => {
                println!("Error: '{}' {}", Red.paint(crate_name), err);
            }
        }

        let found_names = search_crates(crate_name);
        match found_names {
            Ok(found_names) => {
                println!(
                    "{} '{}' {:#?}",
                    Green.paint("Found crate names that contain"),
                    Green.paint(crate_name),
                    found_names
                );
            }
            Err(err) => {
                println!("Error: '{}' {}", Red.paint(crate_name), err);
            }
        }
    }
}

/// Get versions from the local cache of the crates.io index.  
pub fn get_versions(crate_name: &str) -> anyhow::Result<Vec<String>> {
    let mut versions = Vec::new();
    // the linux shell home dir symbol ~ or HOME is not expanded in raw rust
    // I must use the dirs crate
    let mut folder = dirs::home_dir().context("Not home dir")?;
    folder.push(".cargo/registry/index/github.com-1ecc6299db9ec823/.cache");
    //interesting rules for the folder structure
    if crate_name.len() == 1 {
        folder.push("1");
    } else if crate_name.len() == 2 {
        folder.push("2");
    } else if crate_name.len() == 3 {
        folder.push("3");
    } else {
        folder.push(&crate_name[0..2]);
        folder.push(&crate_name[2..4]);
    }
    //println!("Folder: {:?}", &folder);
    let dir = Path::new(&folder);
    if dir.exists() {
        for entry in fs::read_dir(dir)? {
            //crazy amount of unwrap to go from path to a normal string
            let path = entry?.path();
            let entry_file_name = path
                .file_name()
                .context("error file name not exists")?
                .to_str()
                .context("error file name to str")?;
            if entry_file_name == crate_name {
                //read the content and find versions
                let file_content = fs::read_to_string(path)?;
                //I will use regex to find all "vers": "0.3.3",
                let re = Regex::new(r#""vers":"(.*?)".*?"yanked":(.*?)[,}]"#)?;
                for cap in re.captures_iter(&file_content) {
                    //println!("version: {} yanked: {}", &cap[1], &cap[2]);
                    if &cap[2] == "false" {
                        versions.push(cap[1].to_string())
                    }
                }
                break;
            }
        }
    }
    if versions.is_empty() {
        return Err(anyhow!("crate not found"));
    } else {
        //I need the latest first
        versions.reverse();
        //return
        Ok(versions)
    }
}

/// Search local crates.io index cache for a crate name that contains a substring.  
pub fn search_crates(crate_name_substring: &str) -> anyhow::Result<Vec<(String, String)>> {
    let mut folder = dirs::home_dir().context("no home dir")?;
    folder.push(".cargo/registry/index/github.com-1ecc6299db9ec823/.cache");
    let dir = Path::new(&folder);
    let mut found_names: Vec<(String, String)> = Vec::new();
    search_file_name_recursive(&dir, &crate_name_substring, &mut found_names)?;
    //return
    if found_names.is_empty() {
        return Err(anyhow!("crate substring not found"));
    } else {
        Ok(found_names)
    }
}

/// Search for files_names containing a substring, recursive.  
fn search_file_name_recursive(
    dir: &Path,
    file_name_substring: &str,
    found_names: &mut Vec<(String, String)>,
) -> anyhow::Result<()> {
    if dir.is_dir() {
        //println!("This is dir: {:?}", dir.file_name());
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = path
                .file_name()
                .context("no file name improbable")?
                .to_str()
                .context("no to_str improbable")?;
            if path.is_dir() {
                search_file_name_recursive(&path, file_name_substring, found_names)?;
            } else if file_name.contains(file_name_substring) {
                //println!("Found it: {:?}", path);
                //there is no description in this files, only versions and deps
                let v = get_versions(file_name).context("no get version")?;
                //i need only the last version
                let last_version = v.first().context("no first vector")?;
                found_names.push((file_name.to_string(), last_version.to_string()));
            } else {
                //nothing
            }
        }
    }
    return Ok(());
}
