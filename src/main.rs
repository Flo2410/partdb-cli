mod args;

use anyhow::Ok;
use clap::Parser;

use args::{CliArgs, EntityType, FunType};
use partdb_rs::apis::{configuration::Configuration, part_api};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let args = CliArgs::parse();

  let mut config = Configuration::new();
  config.bearer_access_token = Some(args.api_token);
  config.base_path = args.url;

  match args.entity {
    EntityType::Part(entity_args) => match entity_args.function {
      FunType::List => {
        let parts = part_api::api_parts_get_collection(
          &config, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
          None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
          None, None, None, None,
        )
        .await?;

        println!("Here are the parts:");
        println!("Num: {}", parts.hydra_colon_member.len());
        for part in parts.hydra_colon_member.iter() {
          println!("Name: {}", part.name);
          println!("Description: {}", part.description.clone().unwrap_or_default());
          println!("ID: {}", part.id.unwrap_or_default());
          println!("");
        }
      }
      FunType::Show(fun_args) => {
        let part = part_api::api_parts_id_get(&config, fun_args.id.to_string().as_str()).await?;

        if fun_args.raw {
          println!("{}", serde_json::to_string(&part)?);
          return Ok(());
        }

        println!("ID: {}", part.id.unwrap_or_default());
        println!("Name: {}", part.name);
        println!("Description: {}", part.description.unwrap_or_default());
      }
    },
  }

  Ok(())
}
