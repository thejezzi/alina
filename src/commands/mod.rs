pub mod test;

use clap::Subcommand;
use test::TestArgs;

#[derive(Debug, Clone, Subcommand)]
pub enum CommandType {
  Test(TestArgs)
}