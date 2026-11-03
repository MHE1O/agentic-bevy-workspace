use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};

use super::status::{Dimension, parse_scope, replace_scope};

pub(crate) fn validate_game_name(name: &str) -> Result<()> {
    let mut characters = name.chars();
    let valid_first = characters
        .next()
        .is_some_and(|first| first.is_ascii_lowercase());
    let valid_rest = characters.all(|character| {
        character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_'
    });
    if !valid_first || !valid_rest {
        bail!("game name must match [a-z][a-z0-9_]*");
    }
    Ok(())
}

fn copy_directory(source: &Path, destination: &Path) -> Result<()> {
    fs::create_dir(destination).with_context(|| format!("create {}", destination.display()))?;
    for entry in fs::read_dir(source).with_context(|| format!("read {}", source.display()))? {
        let entry = entry?;
        let target = destination.join(entry.file_name());
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            copy_directory(&entry.path(), &target)?;
        } else if file_type.is_file() {
            fs::copy(entry.path(), &target)
                .with_context(|| format!("copy {}", entry.path().display()))?;
        } else {
            bail!(
                "template links and special files are not supported: {}",
                entry.path().display()
            );
        }
    }
    Ok(())
}

fn rewrite_rust_sources(directory: &Path, name: &str) -> Result<()> {
    for entry in fs::read_dir(directory).with_context(|| format!("read {}", directory.display()))? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            rewrite_rust_sources(&path, name)?;
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            let source = fs::read_to_string(&path)
                .with_context(|| format!("read Rust source {}", path.display()))?;
            fs::write(&path, source.replace("game_template", name))
                .with_context(|| format!("write Rust source {}", path.display()))?;
        }
    }
    Ok(())
}

pub(crate) fn create_game(root: &Path, name: &str, dimension: Dimension) -> Result<PathBuf> {
    validate_game_name(name)?;
    if dimension == Dimension::ThreeD {
        bail!("3D scaffolding is not implemented; the current template is intentionally 2D-only");
    }

    let template = root.join("games/_template");
    let destination = root.join("games").join(name);
    if !template.is_dir() {
        bail!("template not found at {}", template.display());
    }
    if destination.exists() {
        bail!("game already exists at {}", destination.display());
    }

    let result = (|| -> Result<()> {
        copy_directory(&template, &destination)?;
        rewrite_rust_sources(&destination, name)?;

        let manifest_path = destination.join("Cargo.toml");
        let manifest = fs::read_to_string(&manifest_path).context("read copied Cargo.toml")?;
        let updated =
            manifest.replacen("name = \"game_template\"", &format!("name = \"{name}\""), 1);
        if updated == manifest {
            bail!("template Cargo.toml has no game_template package name");
        }
        fs::write(&manifest_path, updated).context("write new Cargo.toml")?;

        let scope_path = destination.join("SCOPE.md");
        let document = fs::read_to_string(&scope_path).context("read copied SCOPE.md")?;
        let mut scope = parse_scope(&document)?;
        scope.title = name.to_owned();
        scope.dimension = dimension;
        fs::write(&scope_path, replace_scope(&document, &scope)?).context("write new SCOPE.md")?;
        Ok(())
    })();

    if let Err(error) = result {
        let _ = fs::remove_dir_all(&destination);
        return Err(error);
    }

    println!("Created {} from games/_template", destination.display());
    Ok(destination)
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use tempfile::tempdir;

    use super::{create_game, validate_game_name};
    use crate::tasks::status::{Dimension, parse_scope};

    #[test]
    fn validates_cargo_package_names() {
        for valid in ["game", "my_game2"] {
            validate_game_name(valid).expect("valid name");
        }
        for invalid in ["", "2game", "MyGame", "my-game", "_template", "game/name"] {
            assert!(validate_game_name(invalid).is_err(), "accepted {invalid}");
        }
    }

    #[test]
    fn copies_template_and_rewrites_metadata() {
        let root = tempdir().expect("temporary workspace");
        let template = root.path().join("games/_template");
        fs::create_dir_all(template.join("src")).expect("template source directory");
        fs::create_dir_all(template.join("assets")).expect("template asset directory");
        fs::write(
            template.join("Cargo.toml"),
            "[package]\nname = \"game_template\"\n",
        )
        .expect("template manifest");
        fs::write(
            template.join("SCOPE.md"),
            "---\ntitle: Template Prototype\nphase: Prototype\nprogress: 25\ndimension: 2d\ncurrent_objective: Movement\nblockers: None\n---\n\n# Project\n",
        )
        .expect("template scope");
        fs::write(template.join("src/lib.rs"), "// game\n").expect("template source");
        fs::write(
            template.join("src/main.rs"),
            "game_template::app(false).run();\n",
        )
        .expect("template binary source");
        fs::write(template.join("assets/player.txt"), "asset\n").expect("template asset");

        let destination = create_game(root.path(), "my_game", Dimension::TwoD)
            .expect("create game from template");
        assert_eq!(destination, root.path().join("games/my_game"));
        assert!(destination.join("assets/player.txt").exists());
        let main = fs::read_to_string(destination.join("src/main.rs")).expect("new binary source");
        assert!(main.contains("my_game::app(false).run();"));
        let manifest = fs::read_to_string(destination.join("Cargo.toml")).expect("new manifest");
        assert!(manifest.contains("name = \"my_game\""));
        let scope_text = fs::read_to_string(destination.join("SCOPE.md")).expect("new scope");
        let scope = parse_scope(&scope_text).expect("new scope metadata");
        assert_eq!(scope.title, "my_game");
        assert_eq!(scope.dimension, Dimension::TwoD);
        assert!(scope_text.contains("# Project"));
    }

    #[test]
    fn rejects_3d_before_copying() {
        let root = tempdir().expect("temporary workspace");
        fs::create_dir_all(root.path().join("games/_template")).expect("template");
        assert!(create_game(root.path(), "cube", Dimension::ThreeD).is_err());
        assert!(!Path::new(&root.path().join("games/cube")).exists());
    }
}
