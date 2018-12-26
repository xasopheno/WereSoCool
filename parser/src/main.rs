extern crate colored;
extern crate socool_parser;
use colored::*;
use socool_parser::parser::*;
use std::env;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename;
    if args.len() == 2 {
        filename = &args[1];
    } else {
        println!("\n{}\n", "Forgot to pass in a filename.".red().bold());
        println!("{}", "Example:".cyan());
        println!("{}\n", "./weresocool song.socool".cyan().italic());
        panic!("Wrong number of arguments.")
    }

    let re = Regex::new(r"^\$").unwrap();
    let parsed = parse_file(filename, None);

    for (key, val) in parsed.table.iter() {
        println!("\n Name: {:?}", key);
        //        println!("\nop: {:?}", val);
    }

    //    println!("\n Main: {:?}", parsed.table.get("main").unwrap());
}

#[cfg(test)]
mod test;
