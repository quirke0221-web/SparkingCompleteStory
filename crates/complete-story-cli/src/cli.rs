use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "complete-story-cli",
    version,
    about = "Dragon Ball Sparking! ZERO Complete Story Mod Orchestrator",
    long_about = None,
    propagate_version = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Build mod assets, pack container, and optionally deploy
    Build(BuildArgs),

    /// Pack staged assets into Zen IoStore container
    Pack,

    /// Deploy built mod to Steam ~mods and UE4SS mods folders
    Deploy,

    /// Stream live runtime logs from UE4SS
    Logs,
}

#[derive(Args, Debug)]
pub struct BuildArgs {
    /// Skip Stage 1 extraction if build/staging/legacy is already populated
    #[arg(long)]
    pub skip_extract: bool,

    /// Automatically deploy built containers to Steam ~mods directory
    #[arg(long)]
    pub deploy: bool,
}
