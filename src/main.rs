extern crate glob;
extern crate clap;
extern crate checksum;

use glob::glob_with;
use glob::MatchOptions;

use clap::{Arg, App, AppSettings};
use checksum::crc::Crc as crc;

fn main() {
    let path_arg_name = "path";
    let regular_expression = "expression";
    let args = App::new("check-integrity")
        .about("Check integrity of files.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name(path_arg_name)
            .help("path to the top directory"))
        .arg(Arg::with_name(regular_expression)
            .help("regular expression"))
        .get_matches();

    let mut path = match args.value_of(path_arg_name) {
        Some(path) => path.to_string(),
        None => panic!("You didn't supply a path"),
    };
    let expression = match args.value_of(regular_expression) {
        Some(expression) => expression,
        None => panic!("You didn't supply a regular expression"),
    };

    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    path.push_str("/");
    path.push_str(expression);
    println!("Looking for {:?}\n", path);
    for entry in glob_with(&path, &options).unwrap() {

        if let Ok(path) = entry {

            let mut crc = crc::new(path.to_str().unwrap());
            match crc.checksum() {
                Ok(checksum) => {
                    println!(" {:?}", path.display());
                    println!("   -> CRC32: {:X}", checksum.crc32);
                    println!("   -> CRC64: {:X}", checksum.crc64);
                }
                Err(e) => {
                    println!(" {:?}", path.display());
                    println!("{}", e);
                }
            }
        }

    }
}
