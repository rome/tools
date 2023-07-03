use crate::execute::diagnostics::{ResultExt, ResultIoExt};
use crate::execute::process_file::{DiffKind, FileResult, FileStatus, Message};
use crate::execute::traverse::TraversalOptions;
use crate::execute::TraversalMode;
use crate::FormatterReportFileDetail;
use rome_diagnostics::category;
use rome_fs::{OpenOptions, RomePath};
use rome_service::file_handlers::Language;
use rome_service::workspace::{FileGuard, OpenFileParams};
use std::path::Path;

pub(crate) struct FormatFile<'ctx, 'app> {
    pub(crate) ctx: &'app TraversalOptions<'ctx, 'app>,
    pub(crate) path: &'app Path,
}

pub(crate) fn format_file(payload: FormatFile) -> FileResult {
    let FormatFile { ctx, path } = payload;
    let rome_path = RomePath::new(path);
    let open_options = OpenOptions::default()
        .read(true)
        .write(ctx.execution.requires_write_access());
    let mut file = ctx
        .fs
        .open_with_options(path, open_options)
        .with_file_path(path.display().to_string())?;

    let mut input = String::new();
    file.read_to_string(&mut input)
        .with_file_path(path.display().to_string())?;

    let file_guard = FileGuard::open(
        ctx.workspace,
        OpenFileParams {
            path: rome_path,
            version: 0,
            content: input.clone(),
            language_hint: Language::default(),
        },
    )
    .with_file_path_and_code(path.display().to_string(), category!("internalError/fs"))?;
    let printed = file_guard
        .format_file()
        .with_file_path_and_code(path.display().to_string(), category!("format"))?;

    let output = printed.into_code();
    let should_write = match ctx.execution.traversal_mode {
        TraversalMode::Format { write, .. } => write,
        _ => false,
    };
    ctx.increment_processed();
    if output != input {
        if should_write {
            file.set_content(output.as_bytes())
                .with_file_path(path.display().to_string())?;
            file_guard.change_file(file.file_version(), output)?;
        } else {
            if !ctx.execution.should_report_to_terminal() {
                ctx.push_format_stat(
                    path.display().to_string(),
                    FormatterReportFileDetail {
                        formatted_content: Some(output.clone()),
                    },
                )
            }

            return Ok(FileStatus::Message(Message::Diff {
                file_name: path.display().to_string(),
                old: input,
                new: output,
                diff_kind: DiffKind::Format,
            }));
        }
    }
    Ok(FileStatus::Success)
}
