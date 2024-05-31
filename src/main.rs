mod args;

use anyhow::Ok;
use clap::Parser;

use args::{CliArgs, EntityType, FunType};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let args = CliArgs::parse();

  match args.entity {
    EntityType::Part(part) => match part.function {
      FunType::List => {
        println!("Here are the parts:");
      }
    },
  }

  Ok(())
}
