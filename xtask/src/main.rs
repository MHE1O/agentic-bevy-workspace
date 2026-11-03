mod tasks;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use tasks::status::Dimension;

#[derive(Parser)]
#[command(
    name = "cargo xtask",
    bin_name = "cargo xtask",
    about = "Workspace and agent tooling"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Create a game from games/_template.
    New {
        name: String,
        #[arg(long, value_enum, default_value = "2d")]
        dimension: Dimension,
    },
    /// Print game scope metadata.
    Status,
    /// Build workspace documentation.
    Doc {
        #[arg(long)]
        open: bool,
        #[arg(long)]
        no_deps: bool,
        #[arg(long)]
        private: bool,
    },
    /// Run formatting, Clippy, and tests.
    Verify {
        #[arg(long)]
        game: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask has a workspace parent")
        .to_path_buf();

    match cli.command {
        Command::New { name, dimension } => {
            tasks::new_game::create_game(&root, &name, dimension).map(|_| ())
        }
        Command::Status => tasks::status::run(&root),
        Command::Doc {
            open,
            no_deps,
            private,
        } => tasks::doc::run(&root, open, no_deps, private),
        Command::Verify { game } => tasks::verify::run(&root, game.as_deref()),
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use crate::{Cli, Command, tasks::status::Dimension};

    #[test]
    fn parses_scoped_verification() {
        let cli = Cli::try_parse_from(["xtask", "verify", "--game", "demo"]);
        assert!(matches!(
            cli.map(|cli| cli.command),
            Ok(Command::Verify { game: Some(game) }) if game == "demo"
        ));
    }

    #[test]
    fn new_game_defaults_to_2d() {
        let cli = Cli::try_parse_from(["xtask", "new", "demo"]);
        assert!(matches!(
            cli.map(|cli| cli.command),
            Ok(Command::New { name, dimension }) if name == "demo" && dimension == Dimension::TwoD
        ));
    }
}
