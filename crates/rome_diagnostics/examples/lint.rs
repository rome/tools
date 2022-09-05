use std::io;

use rome_console::{markup, ConsoleExt, EnvConsole};
use rome_diagnostics::v2::{
    Diagnostic, FilePath, IntoAdvices, Location, LogCategory, Path, PrintDiagnostic, SourceCode,
    Visitor,
};
use rome_rowan::{TextRange, TextSize};

#[derive(Debug, Diagnostic)]
#[diagnostic(
    category = "lint/style/noShoutyConstants",
    message = "Redundant constant reference",
    tags(FIXABLE)
)]
struct LintDiagnostic {
    #[location(path)]
    path: String,
    #[location(span)]
    span: TextRange,
    #[location(source_code)]
    source_code: String,
    #[advice]
    advices: LintAdvices,
    #[verbose_advice]
    verbose_advices: LintVerboseAdvices,
}

#[derive(Debug)]
struct LintAdvices {
    path: String,
    declaration_span: TextRange,
    source_code: String,
    fixed_code: String,
}

impl IntoAdvices for LintAdvices {
    fn visit(&self, visitor: &mut dyn Visitor) -> io::Result<()> {
        visitor.visit_log(
            LogCategory::Info,
            &"You should avoid declaring constants with a string that's the same value as the variable name. It introduces a level of unnecessary indirection when it's only two additional characters to inline."
        )?;

        visitor.visit_log(LogCategory::Info, &"This constant is declared here")?;
        visitor.visit_frame(Location {
            path: Path::File(FilePath::Path(&self.path)),
            span: Some(self.declaration_span),
            source_code: Some(SourceCode {
                text: &self.source_code,
                line_starts: None,
            }),
        })?;

        visitor.visit_log(LogCategory::Info, &"Safe Fix")?;
        visitor.visit_diff(&self.source_code, &self.fixed_code)
    }
}

#[derive(Debug)]
struct LintVerboseAdvices {
    path: String,
}

impl IntoAdvices for LintVerboseAdvices {
    fn visit(&self, visitor: &mut dyn Visitor) -> io::Result<()> {
        visitor.visit_log(LogCategory::Info, &"Apply this fix using `--apply`:")?;
        visitor.visit_command(&format!("rome check --apply {}", self.path))
    }
}

pub fn main() {
    const SOURCE: &str = "const FOO = \"FOO\";
console.log(FOO);";

    const FIXED: &str = "console.log(\"FOO\");";

    let diag = LintDiagnostic {
        path: String::from("style/noShoutyConstants.js"),
        span: TextRange::at(TextSize::from(31), TextSize::from(3)),
        source_code: String::from(SOURCE),
        advices: LintAdvices {
            path: String::from("style/noShoutyConstants.js"),
            declaration_span: TextRange::at(TextSize::from(6), TextSize::from(3)),
            source_code: String::from(SOURCE),
            fixed_code: String::from(FIXED),
        },
        verbose_advices: LintVerboseAdvices {
            path: String::from("style/noShoutyConstants.js"),
        },
    };

    EnvConsole::default().error(markup!({ PrintDiagnostic(&diag) }));
}
