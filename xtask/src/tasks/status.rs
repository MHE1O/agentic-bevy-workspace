use std::{fmt, fs, path::Path};

use anyhow::{Context, Result, bail};
use clap::ValueEnum;
use comfy_table::Table;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    Prototype,
    Alpha,
    Polish,
    Complete,
}

impl fmt::Display for Phase {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
pub enum Dimension {
    #[serde(rename = "2d")]
    #[value(name = "2d")]
    TwoD,
    #[serde(rename = "3d")]
    #[value(name = "3d")]
    ThreeD,
}

impl fmt::Display for Dimension {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::TwoD => "2d",
            Self::ThreeD => "3d",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Scope {
    pub title: String,
    pub phase: Phase,
    pub progress: u8,
    pub dimension: Dimension,
    pub current_objective: String,
    pub blockers: String,
}

fn split_frontmatter(document: &str) -> Result<(&str, &str)> {
    let rest = document
        .strip_prefix("---\n")
        .context("SCOPE.md must start with a YAML frontmatter delimiter")?;
    let end = rest
        .find("\n---\n")
        .context("SCOPE.md is missing its closing YAML frontmatter delimiter")?;
    Ok((&rest[..end], &rest[end + 5..]))
}

pub(crate) fn parse_scope(document: &str) -> Result<Scope> {
    let (yaml, _) = split_frontmatter(document)?;
    let scope: Scope = serde_yaml::from_str(yaml).context("invalid SCOPE.md YAML frontmatter")?;
    if scope.progress > 100 {
        bail!("scope progress must be between 0 and 100");
    }
    Ok(scope)
}

pub(crate) fn replace_scope(document: &str, scope: &Scope) -> Result<String> {
    let (_, body) = split_frontmatter(document)?;
    let yaml = serde_yaml::to_string(scope).context("serialize scope metadata")?;
    let yaml = yaml.strip_prefix("---\n").unwrap_or(&yaml);
    Ok(format!("---\n{yaml}---\n{body}"))
}

pub fn run(root: &Path) -> Result<()> {
    let games = root.join("games");
    let mut rows = Vec::new();
    if games.exists() {
        for entry in fs::read_dir(&games).context("read games directory")? {
            let entry = entry?;
            if !entry.file_type()?.is_dir() {
                continue;
            }
            let scope_path = entry.path().join("SCOPE.md");
            if !scope_path.is_file() {
                continue;
            }
            let document = fs::read_to_string(&scope_path)
                .with_context(|| format!("read {}", scope_path.display()))?;
            let scope = parse_scope(&document)
                .with_context(|| format!("parse {}", scope_path.display()))?;
            rows.push((entry.file_name().to_string_lossy().into_owned(), scope));
        }
    }
    rows.sort_by(|left, right| left.0.cmp(&right.0));

    let mut table = Table::new();
    table.set_header(["Game", "Phase", "Progress", "Current Objective", "Blockers"]);
    for (name, scope) in rows {
        table.add_row([
            name,
            scope.phase.to_string(),
            format!("{}%", scope.progress),
            scope.current_objective,
            scope.blockers,
        ]);
    }
    println!("{table}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{Dimension, Phase, parse_scope};

    const VALID: &str = r#"---
title: "Template Prototype"
phase: "Prototype"
progress: 25
dimension: "2d"
current_objective: "Implement movement"
blockers: "None"
---

# Project
"#;

    #[test]
    fn parses_strict_frontmatter() {
        let scope = parse_scope(VALID).expect("valid scope");
        assert_eq!(scope.title, "Template Prototype");
        assert_eq!(scope.phase, Phase::Prototype);
        assert_eq!(scope.progress, 25);
        assert_eq!(scope.dimension, Dimension::TwoD);
    }

    #[test]
    fn rejects_progress_above_one_hundred() {
        let document = VALID.replace("progress: 25", "progress: 101");
        assert!(parse_scope(&document).is_err());
    }

    #[test]
    fn rejects_missing_frontmatter_delimiter() {
        assert!(parse_scope("title: Missing\n").is_err());
    }

    #[test]
    fn rejects_invalid_phase() {
        let document = VALID.replace("Prototype", "Cancelled");
        assert!(parse_scope(&document).is_err());
    }
}
