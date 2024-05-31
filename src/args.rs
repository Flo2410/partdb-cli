use clap::{Args, Parser, Subcommand};

/// A simple CLI for moodle written in Rust.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
  /// Entities
  #[clap(subcommand)]
  pub entity: EntityType,

  /// The PartDB URL
  #[arg(short, long)]
  pub url: String,

  /// The PartDB API token
  #[arg(short, long)]
  pub api_token: String,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
  /// Do actions on parts
  Part(EntityArgs),
}

#[derive(Debug, Args)]
pub struct EntityArgs {
  #[clap(subcommand)]
  pub function: FunType,
}

#[derive(Debug, Subcommand)]
pub enum FunType {
  /// List all entries of the entity
  List,

  /// Show inforamtion about a part
  Show(FunArgs),
}

#[derive(Debug, Args)]
pub struct FunArgs {
  /// The id of the entity
  pub id: u32,
}
