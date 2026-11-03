use std::path::Path;

use anyhow::Result;

use super::run_cargo;

pub(crate) fn fmt_args(game: Option<&str>) -> Vec<&str> {
    let mut args = vec!["fmt"];
    if let Some(game) = game {
        args.extend(["-p", game]);
    }
    args.extend(["--", "--check"]);
    args
}

pub(crate) fn clippy_args(game: Option<&str>) -> Vec<&str> {
    let mut args = vec!["clippy"];
    if let Some(game) = game {
        args.extend(["-p", game]);
    }
    args.extend(["--all-targets", "--", "-D", "warnings"]);
    args
}

pub(crate) fn test_args(game: Option<&str>) -> Vec<&str> {
    let mut args = vec!["test"];
    if let Some(game) = game {
        args.extend(["-p", game]);
    }
    args
}

pub fn run(root: &Path, game: Option<&str>) -> Result<()> {
    run_cargo(root, &fmt_args(game))?;
    run_cargo(root, &clippy_args(game))?;
    run_cargo(root, &test_args(game))
}

#[cfg(test)]
mod tests {
    use super::{clippy_args, fmt_args, test_args};

    #[test]
    fn scopes_every_verification_command() {
        assert_eq!(
            fmt_args(Some("demo")),
            vec!["fmt", "-p", "demo", "--", "--check"]
        );
        assert_eq!(
            clippy_args(Some("demo")),
            vec![
                "clippy",
                "-p",
                "demo",
                "--all-targets",
                "--",
                "-D",
                "warnings"
            ]
        );
        assert_eq!(test_args(Some("demo")), vec!["test", "-p", "demo"]);
    }
}
