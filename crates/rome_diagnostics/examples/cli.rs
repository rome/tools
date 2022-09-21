use std::io;

use rome_console::{markup, ConsoleExt, EnvConsole};
use rome_diagnostics::v2::{Advices, Diagnostic, LogCategory, PrintDiagnostic, Resource, Visit};
use rome_rowan::{TextRange, TextSize};

#[derive(Debug, Diagnostic)]
#[diagnostic(
    category = "flags/invalid",
    message(
        description = "Unknown command {command_name}",
        message("Unknown command "<Emphasis>{self.command_name}</Emphasis>)
    ),
    tags(FIXABLE),
)]
struct CliDiagnostic {
    command_name: String,
    #[location(resource)]
    path: Resource<&'static str>,
    #[location(span)]
    span: TextRange,
    #[location(source_code)]
    source_code: String,
    #[advice]
    advices: CliAdvices,
}

#[derive(Debug)]
struct CliAdvices {
    suggested_name: String,
    suggested_command: String,
}

impl Advices for CliAdvices {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        visitor.record_log(
            LogCategory::Info,
            &markup! {
                "Did you mean "<Emphasis>{self.suggested_name}</Emphasis>" instead?"
            },
        )?;

        visitor.record_command(&self.suggested_command)?;

        visitor.record_log(LogCategory::Info, &"To see all available commands run")?;
        visitor.record_command("rome --help")
    }
}

pub fn main() {
    let diag = CliDiagnostic {
        command_name: String::from("formqt"),
        path: Resource::Argv,
        span: TextRange::new(TextSize::from(5), TextSize::from(11)),
        source_code: String::from("rome formqt file.js"),
        advices: CliAdvices {
            suggested_name: String::from("format"),
            suggested_command: String::from("rome format file.js"),
        },
    };

    EnvConsole::default().error(markup!({ PrintDiagnostic(&diag) }));
}
