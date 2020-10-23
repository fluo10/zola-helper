extern crate clap;
mod cli;
use clap::Clap;
use cli::{Opts, SubCommand, Term, TermSubCommand, TermShow, };


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
        SubCommand::Term(t) => {
            if t.debug {
                println!("Printing debug info...");
            } else {
                println!("Printing normally...");
            }
            match t.subcmd{
                TermSubCommand::Show(t) => {
                    println!("Term show")
                }
            }
        }
    }
//    println!("Using input file: {}", opts.input);
}
