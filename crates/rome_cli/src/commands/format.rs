use std::{fs::read_dir, path::PathBuf};

use rayon::{self, scope, Scope};
use rome_core::App;
use rome_formatter::{format_file_and_save, FormatOptions, IndentStyle};
use rome_path::RomePath;

use crate::CliSession;

pub(crate) fn format(mut session: CliSession) {
    let mut options = FormatOptions::default();

    let size = session
        .args
        .opt_value_from_str("--indent-size")
        .expect("failed to parse indent-size argument");

    let style = session
        .args
        .opt_value_from_str("--indent-style")
        .expect("failed to parse indent-style argument");

    match style {
        Some(IndentStyle::Tab) => {
            options.indent_style = IndentStyle::Tab;
        }
        Some(IndentStyle::Space(default_size)) => {
            options.indent_style = IndentStyle::Space(size.unwrap_or(default_size));
        }
        None => {}
    }

    let inputs = session.args.finish();
    if inputs.is_empty() {
        panic!("needs at least one input file or directory");
    }

    let app = &session.app;
    let options = &options;

    scope(move |scope| {
        for input in inputs {
            scope.spawn(move |scope| {
                handle_path(scope, app, options, input.into());
            });
        }
    });
}

fn handle_path<'a>(scope: &Scope<'a>, app: &'a App, options: &'a FormatOptions, path: PathBuf) {
    if path.is_dir() {
        let iter = match read_dir(path) {
            Ok(iter) => iter,
            Err(_err) => {
                return;
            }
        };

        for entry in iter {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_err) => {
                    continue;
                }
            };

            let path = entry.path();
            scope.spawn(move |scope| {
                handle_path(scope, app, options, path);
            });
        }

        return;
    }

    if path.is_file() {
        let mut file = RomePath::new(path);
        format_file_and_save(&mut file, *options, app);
        return;
    }

    eprintln!("unhandled file type");
}
