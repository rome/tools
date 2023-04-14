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

            if let Some((buffer, _)) = buffer {
                return Ok(buffer
                    .split("\n")
                    .filter(|line| line.len() > 0)
                    .filter_map(|item| {
                        let line = item.to_string();
                        if !line.starts_with("#") {
                            Some(line)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<String>>());
            }
        }
    }

    Ok(vec![])
}
