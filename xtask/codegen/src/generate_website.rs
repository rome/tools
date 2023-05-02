use std::fs;
use xtask::{project_root, Result};

const FRONTMATTER: &str = r#"---
title: VSCode extension
emoji: 💻
category: reference
description: Notes about the Rome's VSCode extension
---
"#;

pub fn generate_website() -> Result<()> {
    fs::remove_file(project_root().join("website/src/pages/vscode.mdx")).ok();
    let readme = fs::read_to_string(project_root().join("editors/vscode/README.md"))?;

    let page = format!("{FRONTMATTER}{readme}");

    fs::write(project_root().join("website/src/pages/vscode.mdx"), page)?;

    Ok(())
}
