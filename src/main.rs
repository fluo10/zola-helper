extern crate clap;
mod cli;
mod cmd;
use cmd::taxonomy;

use clap::Clap;
use cli::{Opts, SubCommand, Taxonomy, TaxonomySubCommand, TaxonomyShow, };


fn main() {
    let opts: Opts = Opts::parse();
    println!("Hello, world!");
    if opts.test {
        println!("Printing  help...");
    } else {
        println!("Not Printing helo...");
    }
    match opts.verbose {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }
    match opts.subcmd {
        SubCommand::Taxonomy(t) => {
            if t.debug {
                println!("Printing debug info...");
            } else {
                println!("Printing normally...");
            }
            match t.subcmd{
                TaxonomySubCommand::Show(t) => {
                    println!("Term show")
                }
            }
        }
    }
//    println!("Using input file: {}", opts.input);
}
