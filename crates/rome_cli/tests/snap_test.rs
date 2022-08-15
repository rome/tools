use rome_console::fmt::{Formatter, Termcolor};
use rome_console::{markup, BufferConsole, Markup};
use rome_diagnostics::termcolor::NoColor;
use rome_fs::{FileSystemExt, MemoryFileSystem};
use std::collections::HashMap;
use std::fmt::Write as _;
use std::path::PathBuf;

#[derive(Default)]
struct InMessages {
    stdin: Option<String>,
}

#[derive(Default)]
struct CliSnapshot {
    /// input messages, coming from different sources
    pub in_messages: InMessages,
    /// the configuration, if set
    pub configuration: Option<String>,
    /// file name -> content
    pub files: HashMap<String, String>,
    /// messages written in console
    pub messages: Vec<String>,
}

impl CliSnapshot {
    fn emit_content_snapshot(&self) -> String {
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
                content.push_str(&**file_content);
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

        if !self.messages.is_empty() {
            content.push_str("# Emitted Messages\n\n");

            for message in &self.messages {
                let message_content = &**message;
                // There are some logs that print the timing, and we can't snapshot that message
                // otherwise at each run we invalid the previous snapshot.
                //
                // This is a workaround, and it might not work for all cases.
                if !message_content.contains("files") {
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

pub fn markup_to_string(markup: Markup) -> String {
    let mut buffer = Vec::new();
    let mut write = Termcolor(NoColor::new(&mut buffer));
    let mut fmt = Formatter::new(&mut write);
    fmt.write_markup(markup).unwrap();

    String::from_utf8(buffer).unwrap()
}

/// Function used to snapshot a session test of the a CLI run.
pub fn assert_cli_snapshot(test_name: &str, fs: MemoryFileSystem, console: BufferConsole) {
    let mut cli_snapshot = CliSnapshot::default();
    let config_path = PathBuf::from("rome.json");
    let configuration = fs.read(&config_path).ok();
    if let Some(mut configuration) = configuration {
        let mut buffer = String::new();
        if configuration.read_to_string(&mut buffer).is_ok() {
            cli_snapshot.configuration = Some(buffer);
        }
    }

    for (file, entry) in fs.files() {
        let content = entry.lock();
        let content = std::str::from_utf8(content.as_slice()).unwrap();

        cli_snapshot
            .files
            .insert(file.to_str().unwrap().to_string(), String::from(content));
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

    let content = cli_snapshot.emit_content_snapshot();

    insta::with_settings!({
        prepend_module_to_snapshot => false,
    }, {
        insta::assert_snapshot!(test_name, content);
    });
}
