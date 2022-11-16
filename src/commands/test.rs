use clap:: {
  Args,
  Parser,
  Subcommand
};
use crate::args::HurrixArgs;

#[derive(Debug, Clone, Parser)]
pub struct TestArgs {
  /// The name of the test
  pub name: Option<String>,

  #[clap(short, long, default_value = "false")]
  pub verbose: bool
}

pub fn exec(args: &TestArgs) {
  
  if args.verbose {
    println!("I am verbose");
  }


  if let Some(name) = &args.name {
    println!("Hello, {}!", name);
  } else {
    println!("Hello, World!");
  }
}