## Linting

We've built Rome to be fantastic at displaying [diagnostics](#diagnostics) and providing as much information as possible for you to understand and fix them.

We aim for Rome to have more opinions, do more for you, while being easy to use. We do this by:

 - Providing as much context as possible on why a diagnostic was produced, including references and explanations for common problems.
 - Offering recommended fixes so you don't need to make most changes yourself (see [recommended fixes](#recommended)).
- Offering suggested fixes for circumstances with multiple or potentially unsafe fixes (see [suggested fixes](#suggested)).
- Integrating with your editor to automatically format code on save, and perform actions on diagnostics such as selecting fix suggestions or adding suppression comments (see [editor integration](#editor-integration)).

### Command Usage

The [`rome check`](#rome-check) command is used to find problems in your project. This includes:

 - Dependency verification
 - Formatting
 - Linting
 - `package.json` validation

We plan on expanding this list to include other checks such as dead code detection, license verification, type checking, and more.

Running `rome check` with no arguments will include all files in your project:

```bash
rome check
```

You can limit this to specific files or directories with:

```bash
rome check App.js components/
```

Rerun automatically every time a file is changed:

```bash
rome check --watch
```

Apply [recommended fixes](#recommended) and [formatting](#formatting):

```bash
rome check --apply
```

Apply only [formatting](#formatting):

```bash
rome check --format-only
```

Choose [suggested fixes](#suggested):

```bash
rome check --review
```

### Rules

Check the full [list of rules](/docs/lint/rules). All rules are enabled, and cannot be disabled. [Suppressions](#suppressions) can be used to hide lint errors.

### Formatting

To use the Rome linter we require usage of the Rome formatter. We offer powerful fixes for most of our lint errors, which can only be done by taking control of code formatting.

Notable formatting choices include:

 - Indentation: Hard tabs. [Improved accessibility](https://github.com/romefrontend/rome/issues/425) over two-spaced tabs.
 - Double string quotes. Consistent quote style across all supported languages.

### Applying Fixes

Rome has two different types of fixes:

#### Recommended

These are always safe to apply and wont risk breaking your code. To apply [formatting](#formatting) and [recommended fixes](#fixing), add the [`--apply`](#--apply) flag:

```bash
rome check --apply
```

#### Suggested

These are for scenarios where there could be multiple ways to fix an issue, or doing so automatically would be unsafe. We include suggestions on some diagnostics for possible fixes. These require an explicit action to apply and can be done via [reviewing](#reviewing).

### Reviewing

All diagnostics have different actions that can be performed. These include applying fix suggestions, adding a suppression comment, and more.

They require an explicit action to apply and can be chosen via the CLI with the `--review` flag on any command:

```bash
rome check --review
```

This displays each [diagnostic](#diagnostics) provides you with a list of actions that you can select using keyboard navigation.

Alternatively, these actions can be applied via a supported [editor integration](#editor-integration).

{% include docs/linting-review.md %}

### Configuration

See [Project Configuration](#project-configuration) for configuration options.

### Diagnostics

Diagnostics are what Rome calls errors. They are emitted absolutely everywhere when Rome finds a problem. This includes CLI argument parsing, JSON normalization, module resolution, lint errors, and more.

#### Anatomy

Diagnostics consist of six main parts:

- The header contains the **filename**, **line**, and **column**. They refer to the position that we believe is the main cause of a problem.
- Followed is the **message** which contains a single-line summary of what we believe is wrong.
- The **code frame** contains a snippet of the file referred in the header. This allows you to see what it's referring to without having to jump into your editor and look it up.
- **Advice** is freeform and appears at the end of a diagnostic. It can include additional messages, lists, other code frames, and more. It gives you more details about why you're seeing the diagnostic, and how you might fix it.

{% include docs/diagnostic-anatomy.md %}

Diagnostics that are fixable via `--apply` are indicated with a label that appears in the header:

{% include docs/diagnostic-anatomy-fixable.md %}

#### Suppressions

Diagnostics can be suppressed with a `rome-ignore` comment, followed by the diagnostic categories you want to suppress, and an optional explanation.

In **JavaScript** this can be a line comment:

```javascript
// rome-ignore lint/js/camelCase
```

In **JavaScript** and **CSS** it can be a block comment:

```javascript
/* rome-ignore lint/js/camelCase */
```

And in **Markdown** and **HTML**:

```html
<!-- rome-ignore categoryName -->
```

##### Enforcement

If a suppression comment does not match suppress at least one diagnostic for every category listed then it will result in an error.

##### Multiple categories

You can suppress multiple categories by separating them with a space.

```javascript
// rome-ignore lint/js/camelCase lint/js/noExplicitAny
```

##### Explanation

You can provide an additional explanation for the suppressed error by prefixing it with a colon:

```javascript
// rome-ignore lint/js/noExplicitAny: Explanation here
```

### Editor Integration

Get the most out of Rome by integrating it with your editor. You will get diagnostics as you type, and saving will automatically format your files.

Rome implements the [Language Server Protocol (LSP)](https://microsoft.github.io/language-server-protocol/) supported by [various editors](https://microsoft.github.io/language-server-protocol/implementors/tools/). We have official extensions available for:

- [VSCode](#)

Once an editor extension has been installed, the version of Rome in your project will be automatically found and used. As we improve Rome and add new functionality any changes will automatically work with your editor!

We welcome contributions adding official extensions for other mainstream editors. See [contributing](/contributing) for more information. LSP communication is done by the [`rome lsp` command](#rome-lsp).
