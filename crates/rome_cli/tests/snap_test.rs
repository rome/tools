use rome_cli::Termination;
use rome_console::fmt::{Formatter, Termcolor};
use rome_console::{markup, BufferConsole, Markup};
use rome_diagnostics::termcolor::NoColor;
use rome_fs::{FileSystemExt, MemoryFileSystem};
use std::collections::BTreeMap;
use std::env::current_exe;
use std::fmt::Write as _;
use std::path::PathBuf;

#[derive(Default)]
struct InMessages {
    stdin: Option<String>,
}

pub(crate) struct CliSnapshot {
    /// input messages, coming from different sources
    in_messages: InMessages,
    /// the configuration, if set
    pub configuration: Option<String>,
    /// file name -> content
    pub files: BTreeMap<String, String>,
    /// messages written in console
    pub messages: Vec<String>,
    /// possible termination error of the CLI
    pub termination: Option<Termination>,
}

impl CliSnapshot {
    pub fn from_result(result: Result<(), Termination>) -> Self {
        Self {
            in_messages: InMessages::default(),
            configuration: None,
            files: BTreeMap::default(),
            messages: Vec::new(),
            termination: result.err(),
        }
    }
}

impl CliSnapshot {
    pub fn emit_content_snapshot(&self) -> String {
        let mut content = String::new();

        if let Some(configuration) = &self.configuration {
            content.push_str("## `rome.json`\n\n");
            content.push_str("```json");
            content.push('\n');
            content.push_str(configuration);
            content.push('\n');
            content.push_str("```");
            content.push_str("\n\n")
        }

        for (name, file_content) in &self.files {
            if !name.starts_with("rome.json") {
                let extension = name.split('.').last().unwrap();

                let _ = write!(content, "## `{name}`\n\n");
                let _ = write!(content, "```{extension}");
                content.push('\n');
                content.push_str(file_content);
                content.push('\n');
                content.push_str("```");
                content.push_str("\n\n")
            }
        }

        if let Some(stdin) = &self.in_messages.stdin {
            content.push_str("# Input messages\n\n");
            content.push_str("```block");
            content.push('\n');
            content.push_str(stdin);
            content.push('\n');
            content.push_str("```");
            content.push_str("\n\n")
        }

        if let Some(termination) = &self.termination {
            let mut message = format!("{:?}", termination);
            let exe = current_exe()
                .ok()
                .and_then(|path| Some(path.file_name()?.to_str()?.to_string()));
            if let Some(exe) = exe {
                if message.contains(&exe) {
                    message = message.replace(&exe, "rome");
                }
            }
            content.push_str("# Termination Message\n\n");
            content.push_str("```block");
            content.push('\n');
            let _ = write!(content, "{}", message);
            content.push('\n');
            content.push_str("```");
            content.push_str("\n\n");
        }

        if !self.messages.is_empty() {
            content.push_str("# Emitted Messages\n\n");

            for message in &self.messages {
                let message_content = &**message;
                // There are some logs that print the timing, and we can't snapshot that message
                // otherwise at each run we invalid the previous snapshot.
                //
                // This is a workaround, and it might not work for all cases.
                if !message_content.contains("files in") {
                    content.push_str("```block");
                    content.push('\n');
                    content.push_str(message_content);
                    content.push('\n');
                    content.push_str("```");
                    content.push_str("\n\n")
                }
            }
        }

        content
    }
}

impl From<SnapshotPayload<'_>> for CliSnapshot {
    fn from(payload: SnapshotPayload<'_>) -> Self {
        let SnapshotPayload {
            result,
            console,
            fs,
            test_name: _,
            module_path: _,
        } = payload;
        let mut cli_snapshot = CliSnapshot::from_result(result);
        let config_path = PathBuf::from("rome.json");
        let configuration = fs.read(&config_path).ok();
        if let Some(mut configuration) = configuration {
            let mut buffer = String::new();
            if configuration.read_to_string(&mut buffer).is_ok() {
                cli_snapshot.configuration = Some(buffer);
            }
        }

        let files: Vec<_> = fs
            .files()
            .into_iter()
            .map(|(file, entry)| {
                let content = entry.lock();
                let content = std::str::from_utf8(content.as_slice()).unwrap();
                (file.to_str().unwrap().to_string(), String::from(content))
            })
            .collect();

        for (file, content) in files {
            cli_snapshot.files.insert(file, content);
        }

        let in_buffer = &console.in_buffer;
        for (index, message) in in_buffer.iter().enumerate() {
            if index == 0 {
                cli_snapshot.in_messages.stdin = Some(message.to_string());
            }
        }

        for message in &console.out_buffer {
            let content = markup_to_string(markup! {
                {message.content}
            });
            cli_snapshot.messages.push(content)
        }

        cli_snapshot
    }
}

pub fn markup_to_string(markup: Markup) -> String {
    let mut buffer = Vec::new();
    let mut write = Termcolor(NoColor::new(&mut buffer));
    let mut fmt = Formatter::new(&mut write);
    fmt.write_markup(markup).unwrap();

    String::from_utf8(buffer).unwrap()
}

pub struct SnapshotPayload<'a> {
    pub module_path: &'a str,
    pub test_name: &'a str,
    pub fs: MemoryFileSystem,
    pub console: BufferConsole,
    pub result: Result<(), Termination>,
}

impl<'a> SnapshotPayload<'a> {
    pub fn new(
        module_path: &'a str,
        test_name: &'a str,
        fs: MemoryFileSystem,
        console: BufferConsole,
        result: Result<(), Termination>,
    ) -> Self {
        Self {
            module_path,
            test_name,
            fs,
            console,
            result,
        }
    }
}

/// Function used to snapshot a session test of the a CLI run.
pub fn assert_cli_snapshot(payload: SnapshotPayload<'_>) {
    let module_path = payload.module_path.to_owned();
    let test_name = payload.test_name;
    let cli_snapshot = CliSnapshot::from(payload);

    let content = cli_snapshot.emit_content_snapshot();

    let module_path = module_path.replace("::", "_");
    let snapshot_path = PathBuf::from("snapshots").join(module_path);

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => snapshot_path
    }, {
        insta::assert_snapshot!(test_name, content);

    });
}
