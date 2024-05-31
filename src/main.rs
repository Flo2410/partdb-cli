mod args;
mod partdb;

use std::str::FromStr;

use anyhow::Ok;
use clap::Parser;

use args::{CliArgs, EntityType, FunType};
use partdb::PartDB;
use url::Url;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let args = CliArgs::parse();

  let partdb = PartDB::new(Url::from_str(&args.url)?, args.api_token)?;

  match args.entity {
    EntityType::Part(part) => match part.function {
      FunType::List => {
        println!("Here are the parts:");
        let parts = partdb.get_parts().await?;
        println!("Num: {}", parts.len());
        for part in parts.iter() {
          println!("Name: {}", part.name);
          println!("Description: {}", part.description);
          println!("ID: {}", part.id);
          println!("");
        }
      }
    },
  }

  Ok(())
}
