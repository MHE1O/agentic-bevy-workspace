use std::{path::Path, process::Command};

use anyhow::{Context, bail};

pub mod doc;
pub mod new_game;
pub mod status;
pub mod verify;

pub(crate) fn run_cargo(root: &Path, args: &[&str]) -> anyhow::Result<()> {
    let status = Command::new("cargo")
        .args(args)
        .current_dir(root)
        .status()
        .with_context(|| format!("failed to launch cargo {}", args.join(" ")))?;
    if !status.success() {
        bail!("cargo {} failed with {status}", args.join(" "));
    }
    Ok(())
}
