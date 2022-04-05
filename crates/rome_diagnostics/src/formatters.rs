use crate::termcolor::{ColorChoice, StandardStream, WriteColor};
use crate::*;
use colored::*;
use file::Files;
use std::collections::HashSet;
use std::io;

/// A trait describing a struct which can render diagnostics to a writer such as stderr.
///
/// Each formatter may rely on behavior specific to a batch of diagnostics, therefore
/// you should collect all diagnostics and then call the appropriate formatter
pub trait Formatter {
    fn emit_stdout(&mut self, diagnostics: &[Diagnostic], files: &dyn Files) -> io::Result<()> {
        let stderr = StandardStream::stderr(ColorChoice::Always);
        let mut out = stderr.lock();
        self.emit_with_writer(diagnostics, files, &mut out)
    }

    fn emit_stderr(&mut self, diagnostics: &[Diagnostic], files: &dyn Files) -> io::Result<()> {
        let stderr = StandardStream::stderr(ColorChoice::Always);
        let mut out = stderr.lock();
        self.emit_with_writer(diagnostics, files, &mut out)
    }

    fn emit_with_writer(
        &mut self,
        diagnostics: &[Diagnostic],
        files: &dyn Files,
        writer: &mut dyn WriteColor,
    ) -> io::Result<()>;
}

#[derive(Debug, Copy, Clone)]
pub struct ShortFormatter;

impl Formatter for ShortFormatter {
    fn emit_with_writer(
        &mut self,
        diagnostics: &[Diagnostic],
        files: &dyn Files,
        writer: &mut dyn WriteColor,
    ) -> io::Result<()> {
        let mut ids = HashSet::new();
        diagnostics.iter().for_each(|d| {
            ids.insert(d.file_id);
        });
        for id in ids {
            let cur_diags = diagnostics
                .iter()
                .filter(|x| x.file_id == id && x.primary.is_some())
                .collect::<Vec<_>>();
            if cur_diags.is_empty() {
                continue;
            }

            let name = files.name(id).expect("Invalid file id");
            writeln!(writer, "{}", name.white().underline())?;
            let mut line_starts = vec![];

            for diag in cur_diags.clone() {
                let line_index = files
                    .line_index(id, diag.primary.as_ref().unwrap().span.range.start)
                    .expect("Line index out of bounds");
                let line_span = files.line_range(id, line_index).unwrap();
                let column = diag.primary.as_ref().unwrap().span.range.start - line_span.start;
                line_starts.push((line_index, column));
            }
            let max_msg_len = cur_diags
                .iter()
                .max_by_key(|x| x.title.trim().len())
                .map(|x| x.title.trim().len())
                .unwrap();

            let max_severity_len = cur_diags
                .iter()
                .map(|x| format!("{:?}", x.severity).len())
                .max()
                .unwrap();

            let max_loc = line_starts
                .iter()
                .map(|x| x.0.to_string().len() + x.1.to_string().len() + 1)
                .max()
                .unwrap();
            for (diag, (line, column)) in cur_diags.into_iter().zip(line_starts) {
                write!(writer, "  ")?;
                write!(
                    writer,
                    "{} ",
                    " ".repeat(max_loc - (line.to_string().len() + column.to_string().len() + 1))
                )?;
                write!(
                    writer,
                    "{}{}{}  ",
                    line.to_string().truecolor(140, 140, 140),
                    ":".truecolor(140, 140, 140),
                    column.to_string().truecolor(140, 140, 140)
                )?;
                let color = match diag.severity {
                    Severity::Bug | Severity::Error => Color::BrightRed,
                    Severity::Note => Color::BrightCyan,
                    Severity::Warning => Color::BrightYellow,
                    Severity::Help => Color::BrightGreen,
                };
                let severity_string = format!("{:?}", diag.severity).to_ascii_lowercase();
                write!(
                    writer,
                    "{}{}  ",
                    " ".repeat(max_severity_len - severity_string.len()),
                    severity_string.color(color)
                )?;
                write!(
                    writer,
                    "{}{}  ",
                    diag.title.trim(),
                    " ".repeat(max_msg_len - diag.title.trim().len())
                )?;
                if let Some(code) = diag.code.clone() {
                    write!(writer, "{}", code.white())?;
                }
                writeln!(writer)?;
            }
            writeln!(writer)?;
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct LongFormatter;

impl Formatter for LongFormatter {
    fn emit_with_writer(
        &mut self,
        diagnostics: &[Diagnostic],
        files: &dyn Files,
        writer: &mut dyn WriteColor,
    ) -> io::Result<()> {
        for diag in diagnostics {
            Emitter::new(files).emit_with_writer(diag, writer)?;
        }
        Ok(())
    }
}
