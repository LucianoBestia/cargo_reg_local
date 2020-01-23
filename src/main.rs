//region: lmake_readme insert "readme.md"
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


//endregion: lmake_readme insert "readme.md"

//region: use statements
use ansi_term::Colour::{Green, Red, Yellow};
use clap::{App, Arg};
use dirs;
use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;
use unwrap::unwrap;
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
        println!("Crate name or substring: {}", Yellow.paint(crate_name));

        let versions = get_versions(&crate_name);
        println!("Versions of crate '{}' {:#?}", crate_name, versions);

        let found_names = search_crates(crate_name);
        println!(
            "Found crate names that contain '{}' {:#?}",
            crate_name, found_names
        );
    }
}

/// Get versions from the local cache of the crates.io index.  
pub fn get_versions(crate_name: &str) -> Vec<String> {
    let mut versions = Vec::new();
    // the linux shell home dir symbol ~ or HOME is not expanded in raw rust
    // I must use the dirs crate
    let mut folder = unwrap!(dirs::home_dir());
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
        for entry in unwrap!(fs::read_dir(dir)) {
            //crazy amount of unwrap to go from path to a normal string
            let path = unwrap!(entry).path();
            let entry_file_name = unwrap!(unwrap!(path.file_name()).to_str());
            if entry_file_name == crate_name {
                //read the content and find versions
                let file_content = unwrap!(fs::read_to_string(path));
                //I will use regex to find all "vers": "0.3.3",
                let re = unwrap!(Regex::new(r#""vers":"(.*?)".*?"yanked":(.*?)[,}]"#));
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
    //I need the latest first
    versions.reverse();
    //return
    versions
}

/// Search local crates.io index cache for a crate name that contains a substring.  
pub fn search_crates(crate_name_substring: &str) -> Vec<(String, String)> {
    let mut folder = unwrap!(dirs::home_dir());
    folder.push(".cargo/registry/index/github.com-1ecc6299db9ec823/.cache");
    let dir = Path::new(&folder);
    let mut found_names: Vec<(String, String)> = Vec::new();
    search_file_name_recursive(&dir, &crate_name_substring, &mut found_names);
    //return
    found_names
}

/// Search for files_names containing a substring, recursive.  
fn search_file_name_recursive(
    dir: &Path,
    file_name_substring: &str,
    found_names: &mut Vec<(String, String)>,
) {
    if dir.is_dir() {
        //println!("This is dir: {:?}", dir.file_name());
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if path.is_dir() {
                search_file_name_recursive(&path, file_name_substring, found_names);
            } else if file_name.contains(file_name_substring) {
                //println!("Found it: {:?}", path);
                //there is no description in this files, only versions and deps
                let v = get_versions(file_name);
                //i need only the last version
                let last_version = unwrap!(v.first());
                found_names.push((file_name.to_string(), last_version.to_string()));
            } else {
                //nothing
            }
        }
    }
}
