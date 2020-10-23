use clap::Clap;

#[derive(Clap)]
#[clap(name = "zola-helper", version = "0.1", author= "fluo10 <fluo10@fireturtle.net>", about = "Helper for handling zola content")]
pub struct Opts {
//  pub input: String,
  #[clap(short, long, parse(from_occurrences), about = "Set the level of verbosity",)]
//  #[clap(short, long, parse(from_occurrences), help_heading = "Set the level of verbosity",)]
  pub verbose: i32,
  #[clap(short, long,)]
  pub test: bool,
  #[clap(subcommand)]
  pub subcmd: SubCommand,
}
#[derive(Clap)]
pub enum SubCommand{
  #[clap(version = "0.1", author= "fluo10 <fluo10@fireturtle.net>", about = "Controls taxonomy term", )]
  Term(Term),
}

#[derive(Clap)]
pub struct Term {
    /// Print debug info
    #[clap(short)]
    pub debug: bool,
    #[clap(subcommand)]
    pub subcmd: TermSubCommand,
}
#[derive(Clap)]
pub enum TermSubCommand{
  #[clap(version = "0.1", author= "fluo10 <fluo10@fireturtle.net>", about = "Controls taxonomy term", )]
  Show(TermShow),
}

#[derive(Clap)]
pub struct TermShow {
    /// Print debug info
    #[clap(short)]
    pub debug: bool,
}
    


