# Writing Diagnostics

The `rome_diagnostics` crate implements "Diagnostics", the generic concept of
how the various errors and issues that can be raised in the Rome codebase, 
and displayed to the final user. This guide is aimed at
contributors to the Rome codebase to help writing diagnostics, both at the
technical level, and as general best practices to make diagnostics easier to
understand for the final user.

## What is a Diagnostic

At the lowest level, `Diagnostic` is a Rust trait implemented by various types
across the codebase that each represent a specific kind of diagnostic. All
these types have the following properties in common, as enforced by the trait:

- A `category`, the unique string of the error code associated with this
diagnostic type such as `lint/correctness/noArguments`, `args/invalid` or
`format/disabled`
- A `severity` defining how important the issue described by the diagnostic is
to the user. The severity of a diagnostic can be `Fatal`, `Error`, `Warning`,
`Information` or `Hint`
- A `description` text explaining the diagnostic to the user. This is what will
be displayed in contexts that do not support rich markup, such as embedded
error popovers in editors. Since the description is shown in contexts where the
advices cannot be rendered, it should provide as much information as possible about the
error.
- A `message`, a piece of rich markup that will be displayed at the top of the
diagnostic advices. Contrary to the description, the message is displayed along
with the other advices of the diagnostic and should only provide a short and
clear explanation of the error, while advices will be used to provide
additional information with the appropriate context.
- A set of `advices`, the main building blocks used to provide the user with
rich information about the diagnostic. Depending on the use case, advices can be
used to log pieces of markup, lists of items, annotated code frames, text diffs,
Rust backtraces, command lines, or groups of more advices.
- An optional set of `verbose_advices` the user can optionally enable to get
further information about a given diagnostic
- A `location` describing where the error happened. It can be a path to a file
on the file system, a command line argument, or an arbitrary memory buffer. It may
optionally specify a specific range within the text content of this resource,
as well as embed said text content to faciliate it's retrieval when displaying
code frames in the diagnostic
- `tags` conveying additional informations about the diagnostic: if the
diagnostic has information on how it can be fixed, if it resulted from an
internal error in Rome and was not directly caused by the user, if it is being
emitted to warn the user about unused or deprecated code.
- An optional `source`, another diagnostic that details the low-level reason
why this diagnostic was emitted: for instance a diagnostic reporting a failed
request to a remote server may have a deserialization error for the server
response as its cause.

## How to implement Diagnostic

In theory a diagnostic is created by implementing the `Diagnostic` trait on a
type. In practice since there's a lot of methods to implement, the
`rome_diagnostics` crate exposes a `derive` procedural macro to make
implementing the trait easier:

```rust
// The Diagnostic trait requires Debug to be implemented
#[derive(Debug, Diagnostic)]
// The category, severity, description, message, location and tags can be
// specified statically on the type itself using the #[diagnostic] attribute
#[diagnostic(severity = Warning, category = "internalError/fs")]
struct UnhandledDiagnostic {
    // All the diagnostic properties can also be derived from fields of the
    // struct using the corresponding attribute
    // A single field may have multiple attributes, however most attributes can
    // only be specified once either statically on the whole struct or on a
    // single field. The only exception to this is #[advice] (and
    // #[verbose_advice]), since all advices will be recorded into the
    // diagnostic in the same order they are declared in the struct
    #[message]
    #[description]
    #[advice]
    file_kind: UnhandledKind,
    // For the location, it's possible to specify a sub-property between
    // `resource`, `span` and `source_code`
    #[location(resource)]
    file_id: FileId,
}
```

This should be enough to define most of the properties of a diagnostic, but
advices are a bit more complex. Fields that have the `#[advice]` or
`#[verbose_advice]` attribute are expected to implement the `Advices` trait.
This trait allows arbitrary types to record advices on the diagnostic that
contains them:

```rust
impl Advices for UnhandledKind {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        visitor.record_log(LogCategory::Info, self.advice_message())
    }
}
```

The category may also require some special care if you're declaring a new one,
since all diagnostic categories have to be statically registered you'll need to
add it to `crates/rome_diagnostics_categories/src/categories.rs`

## Writing a Diagnostic

Diagnostics are at the core of the experience of the Rome toolchain for the
users, and providing high quality diagnostics is crucial to making the usage of
Rome as frictionless as possible even when errors happen. What follows is a
list of best practice for writing good diagnostics:

- A diagnostic should not simply state that something went wrong, it should
explain why it went wrong. Add explanations in log advices, and show hyperlinks
to relevant documentation pages in case the user wants to know more.
- If possible, a diagnostic should also try to provide a way for the user to
fix the issue. This can be in the form of a simple log advice, as a diff advice
for a source code change, or as a command advice to prompt the user for a
direct action. In any case, don't forget to add the `FIXABLE` tag to the
diagnostic to highlight to the user it contains an actionable hint.
- Show don't tell: while log advices are highly versatile, always prefer
showing a rich advice like a code frame, diff or command to a textual
explanation if you can, since those are generally easier to understand and may
provide additional context.
