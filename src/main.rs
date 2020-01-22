//! **cargo_reg_local - find data from crates.io in local cache**  
//region: lmake_readme insert "readme.md"

//endregion: lmake_readme insert "readme.md"

//region: use statements
use ansi_term::Colour::{Green, Red, Yellow};
use clap::{App, Arg};
use dirs;
use std::env;
use std::fs;
use std::path::Path;
//use unwrap::unwrap;
//endregion

#[allow(clippy::print_stdout, clippy::integer_arithmetic)]
/// The program starts here. Linux only. Argument crate name.
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

    let mut crate_name = "";
    if let Some(c) = matches.value_of("crate_name") {
        crate_name = c;
        println!("Value for crate_name: {}", Yellow.paint(crate_name));
    }
    if !crate_name.is_empty() {
        // the linux shell home dir symbol ~ is not expanded in raw rust
        let mut folder = dirs::home_dir().unwrap();
        folder.push(".cargo/registry/index/github.com-1ecc6299db9ec823/.cache/");

        let dir = Path::new(&folder);
        let found = find_file_recursive(dir, crate_name);
        println!("found: {}", Green.paint(&found));
        if !found.is_empty() {
            //read the content and maybe deserialize it somehow
        }
    }
}

/// find file recursive
/// TODO: return result and errors. Now I return always empty string
fn find_file_recursive(dir: &Path, file_name: &str) -> String {
    if dir.is_dir() {
        //println!("This is dir: {:?}", dir.file_name());
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_dir() {
                let ret_val = find_file_recursive(&path, file_name);
                if !ret_val.is_empty() {
                    return ret_val;
                }
            } else if path
                .file_name()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap()
                == file_name
            {
                println!("Found it: {:?}", path);
                return path.to_str().unwrap().to_owned();
            } else {
                //println!("entry: {:?}", path.file_name())
                //nothing
            }
        }
    } else {
        println!("Path is not a directory: {:?}", dir);
        return "".to_owned();
        /*
        Err(format!(
            "Path is not a directory: {:?}",
            dir.file_name().unwrap()
        ))
        */
    }
    //println!("File_name {} not in here : {:?}",file_name,dir.file_name().unwrap());
    return "".to_owned();
}
