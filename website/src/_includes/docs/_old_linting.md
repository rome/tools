## Linting

We've built Rome to be fantastic at displaying [diagnostics](#diagnostics). When we show you an error we want to give you all the information you need to understand why it appeared, how you can fix it, and how to avoid it in the future.

Rome stands out in the following ways:

**Rich UI:** Our diagnostic format allows us to show you rich information with formatting. This includes line wrapping in the terminal, syntax highlighting, lists, hyperlinks, and more.

**Fixes:** We provide [fixes](#applying-fixes) for many lint errors, which can be applied automatically. If there are multiple ways to fix something then we [suggest](#suggested) multiple fixes that you can choose.

**Reviewing:** We offer an [interactive CLI command](#reviewing) to make this process even easier. It allows you to go through each diagnostic and perform actions on them such as inserting a suppression comment or applying a specific fix.

**Editor:** You can use an [editor integration](#editor-integration) to bring the power of Rome into your editor. This includes lint errors as you type, automatic formatting when saved, and code actions to select specific fixes.

**Safety:** We save a copy of all files before we modify them and cache them. This cache can be managed with the [`rome recover` command](#rome-recover). You will always be able to revert when Rome modifies your code even without a commit history.

{% include docs/cli-screenshots/check.md %}

### Command Usage

The [`rome check`](#rome-check) command is used to find problems in your project. This includes:

- formatting
- linting

We plan on expanding this list to include other checks such as dead code detection,
license verification, type checking, dependency verification, `package.json` and more.

Running `rome check` with no arguments will include all files in your project:

```bash
rome check
```

You can limit this to specific files or directories with:

```bash
rome check App.js components/
```

Rerun automatically every time a file changes:

```bash
rome check --watch
```

Apply [safe fixes](#safe-fixes) and [formatting](#formatting):

```bash
rome check --apply
```

Apply only [formatting](#formatting):

```bash
rome check --format-only
```

Choose [suggested fixes](#suggested-fixes):

```bash
rome check --review
```

### Rules

We have support for over 100 rules, including the most common rules needed working with TypeScript and React.

**See the full [list of rules](/docs/lint/rules).**

All rules are enabled by default, and cannot be disabled. [Suppressions](#suppressions) can be used to hide specific lint errors.

### Formatting

To use the Rome linter we require usage of the Rome formatter. We offer powerful fixes for most of our lint errors, which can only be done by taking control of code formatting.

Notable formatting choices include:

 - Indentation: Hard tabs. [Improved accessibility](https://github.com/rome/tools/issues/425) over two-spaced tabs.
	 You can change the indentation type in the configuration file.
 - Double string quotes. Consistent quote style across all supported languages.

### Applying Fixes

Rome has two different types of fixes:

#### Safe Fixes

For some lint errors, the fixes are unambiguous and can be applied automatically. Diagnostics that are fixable are indicated with a label that appears in the header:

{% include docs/diagnostic-anatomy-fixable.md %}

To apply safe fixes and [formatting](#formatting), add the [`--apply`](#--apply) flag:

```bash
rome check --apply
```

#### Suggested Fixes

These are for scenarios where there could be multiple ways to fix an issue, or doing so automatically would be unsafe. We include suggestions on some diagnostics for possible fixes. These require an explicit action to apply and can be done via [reviewing](#reviewing).

{% include docs/cli-screenshots/lint-suggestions.md %}

### Reviewing

All diagnostics have different actions that can be performed. These include applying fix suggestions, adding a suppression comment, and more.

They require an explicit action to apply and can be chosen via the CLI with the `--review` flag on any command:

```bash
rome check --review
```

This displays each [diagnostic](#diagnostics) and provides you with a list of actions that you can select using keyboard navigation.

Alternatively, these actions can be applied via a supported [editor integration](#editor-integration).

{% include docs/cli-screenshots/lint-review.md %}

### Configuration

See [Project Configuration](#project-configuration) for configuration options.

### Diagnostics

A diagnostic that belong to a `lint` (e.g. `lint/js/noUnusedVariables`) category is built in three parts divided by a slash (/):

- `lint`, the category of the diagnostic
- `js`, the **lint category** which tells which kind of rule is throwing the error
- `noUnusedVariables`, the **name** of the rule

#### Suppressions

Diagnostics can be suppressed with a `rome-ignore` comment. Comments must be followed by the diagnostic categories you want to suppress and a mandatory explanation.

In **JavaScript** this can be a line comment:

```javascript
// rome-ignore lint/js/useCamelCase: match upstream library casing
```

In **JavaScript** and **CSS** it can be a block comment:

```javascript
/* rome-ignore lint/js/useCamelCase: match upstream library casing  */
```

And in **Markdown** and **HTML**:

```html
<!-- rome-ignore html/useClosingNonVoid: allow self-closing divs -->
```

##### Enforcement

If a suppression comment does not match suppress at least one diagnostic for every category listed then it will result in an error.

##### Multiple categories

You can suppress multiple categories by separating them with a space.

```javascript
// rome-ignore lint/js/useCamelCase lint/js/noExplicitAny
```

##### Explanation

You must provide an additional explanation for the suppressed error by prefixing it with a colon:

```javascript
// rome-ignore lint/js/noExplicitAny: explanation here
```

### Editor Integration

Get the most out of Rome by integrating it with your editor. You will get diagnostics as you type, and saving will automatically format your files.

Rome implements the [Language Server Protocol (LSP)](https://microsoft.github.io/language-server-protocol/) supported by [various editors](https://microsoft.github.io/language-server-protocol/implementors/tools/). We have official extensions available for:

- [VSCode](https://marketplace.visualstudio.com/items?itemName=rome.rome)

Once an editor extension has been installed, the version of Rome in your project will be automatically found and used. As we improve Rome and add new functionality any changes will automatically work with your editor!

We welcome contributions adding official extensions for other mainstream editors. See [contributing](https://github.com/rome/tools/blob/main/CONTRIBUTING.md) for more information. LSP communication is done by the [`rome lsp` command](#rome-lsp).
