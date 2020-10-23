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
  Taxonomy(Taxonomy),
}

#[derive(Clap)]
pub struct Taxonomy {
    /// Print debug info
    #[clap(short)]
    pub debug: bool,
    #[clap(subcommand)]
    pub subcmd: TaxonomySubCommand,
}
#[derive(Clap)]
pub enum TaxonomySubCommand{
  #[clap(version = "0.1", author= "fluo10 <fluo10@fireturtle.net>", about = "Controls taxonomy term", )]
  Show(TaxonomyShow),
}

#[derive(Clap)]
pub struct TaxonomyShow {
    /// Print debug info
    #[clap(short)]
    pub debug: bool,
}
    


