use crate::vcs::VcsClient;
use crate::CliDiagnostic;
use std::io::Read;
use std::process::{Command, Stdio};

pub(crate) struct Git;

impl Git {
    const COMMAND: &'static str = "git";
}

impl VcsClient for Git {
    fn changed_files(&self) -> Result<Vec<String>, CliDiagnostic> {
        let process = Command::new(Git::COMMAND)
            .arg("diff")
            .arg("--name-only")
            .stdout(Stdio::piped())
            .spawn()?;

        if let Some(mut output) = process.stdout {
            let mut buffer = String::new();
            output.read_to_string(&mut buffer)?;
            Ok(buffer.lines().map(String::from).collect::<Vec<_>>())
        } else {
            Ok(Vec::new())
        }
    }
}
