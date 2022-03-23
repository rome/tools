//! Implementation of converting, and emitting diagnostics
//! using `codespan`.

use crate::codespan;
use crate::codespan::diagnostic::{Diagnostic as CodespanDiag, Label, LabelStyle, Note, Severity};
use crate::codespan::files::Error;
use crate::codespan::term::{
    emit,
    termcolor::{ColorChoice, StandardStream, WriteColor},
    *,
};
use crate::{
    file::{Files, SimpleFile},
    suggestion::*,
    Diagnostic,
};
use rome_text_edit::*;
use std::{collections::HashMap, ops::Range};

#[derive(Copy, Clone, PartialEq)]
enum EmitterFileId {
    Virtual(usize),
    Real(usize),
}

struct EmitterFiles<'files> {
    real_files: &'files dyn Files,
    virtual_files: HashMap<usize, SimpleFile>,
}

impl<'a> codespan::files::Files<'a> for EmitterFiles<'_> {
    type FileId = EmitterFileId;
    type Source = &'a str;
    type Name = &'a str;

    fn name(&self, id: EmitterFileId) -> Result<&str, Error> {
        match id {
            EmitterFileId::Real(real) => self.real_files.name(real).ok_or(Error::FileMissing),
            EmitterFileId::Virtual(_) => Ok("why are you seeing this 🤔, go yell at the developer"),
        }
    }

    fn source(&self, id: EmitterFileId) -> Result<&str, Error> {
        match id {
            EmitterFileId::Real(real) => self.real_files.source(real).ok_or(Error::FileMissing),
            EmitterFileId::Virtual(id) => self
                .virtual_files
                .get(&id)
                .and_then(|x| x.source(id))
                .ok_or(Error::FileMissing),
        }
    }

    fn line_index(&self, file_id: EmitterFileId, byte_index: usize) -> Result<usize, Error> {
        match file_id {
            EmitterFileId::Real(real) => {
                self.real_files
                    .line_index(real, byte_index)
                    .ok_or(Error::IndexTooLarge {
                        given: byte_index,
                        max: usize::MAX,
                    })
            }
            EmitterFileId::Virtual(id) => self
                .virtual_files
                .get(&id)
                .and_then(|x| x.line_index(id, byte_index))
                .ok_or(Error::IndexTooLarge {
                    given: byte_index,
                    max: usize::MAX,
                }),
        }
    }

    fn line_range(&self, id: EmitterFileId, line_index: usize) -> Result<Range<usize>, Error> {
        match id {
            EmitterFileId::Real(real) => {
                self.real_files
                    .line_range(real, line_index)
                    .ok_or(Error::IndexTooLarge {
                        given: line_index,
                        max: usize::MAX,
                    })
            }
            EmitterFileId::Virtual(id) => self
                .virtual_files
                .get(&id)
                .and_then(|x| x.line_range(id, line_index))
                .ok_or(Error::IndexTooLarge {
                    given: line_index,
                    max: usize::MAX,
                }),
        }
    }
}

fn default_config() -> Config {
    let mut config = Config::default();
    config.chars.multi_top_left = '┌';
    config.chars.multi_bottom_left = '└';
    config
}

/// The emitter is responsible for emitting
/// diagnostics to a given output.
pub struct Emitter<'files> {
    files: &'files dyn Files,
}

impl<'files> Emitter<'files> {
    /// Creates a new `Emitter`.
    pub fn new(files: &'files dyn Files) -> Self {
        Self { files }
    }
}

impl Emitter<'_> {
    /// Render and emit the diagnostic to stderr
    ///
    /// This method will lock stderr for the entire time it takes to emit the diagnostic.
    pub fn emit_stderr(&mut self, d: &Diagnostic, color: bool) -> Result<(), Error> {
        let out = StandardStream::stderr(if color {
            ColorChoice::Always
        } else {
            ColorChoice::Never
        });
        let mut out = out.lock();
        self.emit_with_writer(d, &mut out)
    }

    /// Render and emit the diagnostic to stdout
    ///
    /// This method will lock stdout for the entire time it takes to emit the diagnostic.
    pub fn emit_stdout(&mut self, d: &Diagnostic, color: bool) -> Result<(), Error> {
        let out = StandardStream::stdout(if color {
            ColorChoice::Always
        } else {
            ColorChoice::Never
        });
        let mut out = out.lock();
        self.emit_with_writer(d, &mut out)
    }

    pub fn emit_with_writer(
        &mut self,
        d: &Diagnostic,
        writer: &mut dyn WriteColor,
    ) -> Result<(), Error> {
        let labels = d
            .children
            .iter()
            .chain(d.primary.as_ref())
            .map(|sub| {
                let style = if sub.severity == Severity::Bug || sub.severity == Severity::Error {
                    LabelStyle::Primary
                } else {
                    LabelStyle::Secondary
                };
                Label::new(
                    style,
                    EmitterFileId::Real(d.file_id),
                    sub.span.range.clone(),
                )
                .with_message(sub.msg.clone())
            })
            .collect::<Vec<_>>();

        let mut diagnostic = CodespanDiag {
            file_id: EmitterFileId::Real(d.file_id),
            severity: d.severity,
            labels,
            code: d.code.clone(),
            message: d.title.clone(),
            notes: vec![],
            anonymous: false,
            render_extra_empty: false,
        };
        let mut additional_diags = vec![];
        let mut virtual_files = HashMap::new();
        let mut notes = d
            .footers
            .clone()
            .into_iter()
            .map(|x| Note {
                message: x.msg,
                severity: Some(x.severity),
            })
            .collect::<Vec<_>>();

        for (idx, suggestion) in d.suggestions.iter().enumerate() {
            diagnostic.render_extra_empty = true;
            let replacement = match &suggestion.substitution {
                SuggestionChange::Indels(indels) => {
                    let mut old = self
                        .files
                        .source(suggestion.span.file)
                        .expect("Non existant file id")[suggestion.span.range.clone()]
                    .to_owned();
                    apply_indels(indels, &mut old);
                    old
                }
                SuggestionChange::String(string) => string.clone(),
            };

            match suggestion.style {
                SuggestionStyle::Full => {
                    let labels = suggestion
                        .labels
                        .iter()
                        .map(|x| {
                            Label::new(LabelStyle::Primary, EmitterFileId::Virtual(idx), x.clone())
                        })
                        .collect();
                    let diag = CodespanDiag {
                        file_id: EmitterFileId::Real(d.file_id),
                        severity: Severity::Help,
                        message: suggestion.msg.clone(),
                        code: None,
                        anonymous: true,
                        notes: vec![],
                        labels,
                        render_extra_empty: false,
                    };
                    additional_diags.push(diag);
                    let mut cloned = self
                        .files
                        .source(suggestion.span.file)
                        .expect("Non-existant file id in suggestion")
                        .to_string();
                    cloned.replace_range(suggestion.span.range.clone(), &replacement);
                    let file = SimpleFile::new(
                        "why are you seeing this 🤔, go yell at the developer".to_string(),
                        cloned,
                    );
                    virtual_files.insert(idx, file);
                }
                SuggestionStyle::Inline => {
                    notes.push(Note {
                        message: format!("{}: `{}`", suggestion.msg, replacement),
                        severity: Some(Severity::Help),
                    });
                }
                SuggestionStyle::HideCode => {
                    notes.push(Note {
                        message: suggestion.msg.clone(),
                        severity: Some(Severity::Help),
                    });
                }
                SuggestionStyle::DontShow => {}
            }
        }

        if additional_diags.is_empty() {
            diagnostic.notes.extend(notes);
        } else {
            additional_diags.last_mut().unwrap().notes.extend(notes);
        }
        let iter = std::iter::once(diagnostic).chain(additional_diags.into_iter());

        let files = EmitterFiles {
            real_files: self.files,
            virtual_files,
        };

        // If the diagnostics has no primary label, print it in medium mode instead of rich
        let mut config = default_config();
        if d.primary.is_none() {
            config.display_style = DisplayStyle::Medium;
        }

        for diag in iter {
            emit(writer, &config, &files, &diag)?;
        }
        writer.write(b"\n").map(|_| ()).map_err(Error::Io)
    }
}
