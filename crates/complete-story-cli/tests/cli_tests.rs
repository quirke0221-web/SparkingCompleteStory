use clap::Parser;
use complete_story_cli::cli::{Cli, Commands};

#[test]
fn test_cli_parse_build_defaults() {
    let args = ["complete-story-cli", "build"];
    let cli = Cli::try_parse_from(args).expect("Should parse build command");
    match cli.command {
        Commands::Build(build_args) => {
            assert!(!build_args.skip_extract);
            assert!(!build_args.deploy);
        }
        _ => panic!("Expected Build command"),
    }
}

#[test]
fn test_cli_parse_build_with_flags() {
    let args = ["complete-story-cli", "build", "--skip-extract", "--deploy"];
    let cli = Cli::try_parse_from(args).expect("Should parse build with flags");
    match cli.command {
        Commands::Build(build_args) => {
            assert!(build_args.skip_extract);
            assert!(build_args.deploy);
        }
        _ => panic!("Expected Build command"),
    }
}

#[test]
fn test_cli_parse_pack() {
    let args = ["complete-story-cli", "pack"];
    let cli = Cli::try_parse_from(args).expect("Should parse pack");
    assert!(matches!(cli.command, Commands::Pack));
}

#[test]
fn test_cli_parse_deploy() {
    let args = ["complete-story-cli", "deploy"];
    let cli = Cli::try_parse_from(args).expect("Should parse deploy");
    assert!(matches!(cli.command, Commands::Deploy));
}

#[test]
fn test_cli_parse_logs() {
    let args = ["complete-story-cli", "logs"];
    let cli = Cli::try_parse_from(args).expect("Should parse logs");
    assert!(matches!(cli.command, Commands::Logs));
}
