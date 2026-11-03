use std::path::Path;

use anyhow::Result;

use super::run_cargo;

pub(crate) fn doc_args(open: bool, no_deps: bool, private: bool) -> Vec<&'static str> {
    let mut args = vec!["doc"];
    if open {
        args.push("--open");
    }
    if no_deps {
        args.push("--no-deps");
    }
    if private {
        args.push("--document-private-items");
    }
    args
}

pub fn run(root: &Path, open: bool, no_deps: bool, private: bool) -> Result<()> {
    run_cargo(root, &doc_args(open, no_deps, private))
}

#[cfg(test)]
mod tests {
    use super::doc_args;

    #[test]
    fn maps_documentation_flags() {
        assert_eq!(
            doc_args(true, true, true),
            vec!["doc", "--open", "--no-deps", "--document-private-items"]
        );
        assert_eq!(doc_args(false, false, false), vec!["doc"]);
    }
}
