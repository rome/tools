use crate::diagnostics::NoVcsFolderFound;
use crate::{CliDiagnostic, CliSession};
use rome_service::configuration::vcs::{VcsClientKind, VcsConfiguration};
use rome_service::WorkspaceError;
use std::path::PathBuf;

pub(crate) fn read_vcs_ignore_file(
    session: &mut CliSession,
    current_directory: PathBuf,
    configuration: &VcsConfiguration,
) -> Result<Vec<String>, CliDiagnostic> {
    if configuration.enabled == false {
        return Ok(vec![]);
    }
    let file_system = &session.app.fs;
    // let working_directory = file_system.working_directory();
    // let current_directory = match working_directory {
    //     Some(wd) => wd,
    //     None => PathBuf::new(),
    // };
    // let current_directory = match &configuration.root {
    //     None => current_directory,
    //     Some(root) => {
    //         if root == "/" {
    //             current_directory
    //         } else {
    //             current_directory.join(root)
    //         }
    //     }
    // };

    dbg!(&current_directory);

    if let Some(client_kind) = &configuration.client_kind {
        match client_kind {
            VcsClientKind::Git => {
                let git_folder = current_directory.join(".git");
                if !git_folder.exists() {
                    return Err(CliDiagnostic::NoVcsFolderFound(NoVcsFolderFound {
                        path: git_folder.display().to_string(),
                    }));
                }
            }
        }
        if matches!(configuration.use_ignore_file, Some(true)) {
            let buffer = file_system
                .auto_search(current_directory, client_kind.ignore_file(), false)
                .map_err(WorkspaceError::from)?;

            if let Some(buffer) = buffer {
                return Ok(buffer
                    .split("\n")
                    .map(|item| item.to_string())
                    .collect::<Vec<String>>());
            }
        }
    }

    Ok(vec![])
}
