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
    EntityType::Part(entity_args) => match entity_args.function {
      FunType::List => {
        let parts = partdb.get_parts().await?;

        println!("Here are the parts:");
        println!("Num: {}", parts.len());
        for part in parts.iter() {
          println!("Name: {}", part.name);
          println!("Description: {}", part.description);
          println!("ID: {}", part.id);
          println!("");
        }
      }
      FunType::Show(fun_args) => {
        let part = partdb.get_part(fun_args.id).await?;

        println!("ID: {}", fun_args.id);
        println!("Name: {}", part.name);
        println!("Description: {}", part.description);
      }
    },
  }

  Ok(())
}
