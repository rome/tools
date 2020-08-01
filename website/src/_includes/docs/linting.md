## Linting

We've built Rome to be fantastic at displaying [diagnostics](#diagnostics) and providing as much information as possible for you to understand and fix them. We don't believe that existing JavaScript linters do enough, and get in the way more than they should.

We aim for Rome to have more opinions, do more for you, while being easy to use. We try to achieve this the following ways:

 - Provide as much context as possible on why a diagnostic was produced.
 - Make it obvious and tell you how to fix it (if possible).
 - Offer powerful autofixes so you don't even need to make most changes yourself (via [fixes]()).
 - Offer autofix suggestions for scenarios for potentially unsafe fixes (via [suggestions]()).
 - Make it easy to review all diagnostics and perform actions on them (via [review]()).

To use the Rome linter, you also need to use the autoformatter. Our autofixes rely on being able to modify your code in a rich way which requires being able to consistently format the result.

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

To apply [formatting](#formatting) and [recommended fixes](#fixing), add the [`--apply`](#--apply) flag:

```bash
rome check --apply
```

To enter an interactive [review mode](#reviewing-diagnostics), add the [`--review`](#--review) flag. This displays each [diagnostic](#diagnostics) and allows you to perform actions on them such as automatically fixing, adding a suppression comment, choosing a specific fix suggestion, and more.

```bash
rome check --review
```

Or, if you'd like to run `check` automatically every time a file is changed, add the `--watch` flag:

```bash
rome check --watch
```

### Rules

Check the full [list of rules](/docs/lint/rules).


### Formatting

To use the Rome linter we require usage of the Rome formatter. We offer powerful autofixes for most of our lint errors, which can only be done by taking control of code formatting.

 - Indentation: Hard tabs. [Improved accessibility](https://github.com/romefrontend/rome/issues/425) over two-spaced tabs.
 - String quotes: Double. Consistent quote style across all supported languages.

### Fixing

Rome has two different types of autofixes that can be applied:

1. **Recommended fixes.** These are changes we are confident we can apply for all cases and generally safe. They are generally basic 1:1 .
2. **Suggested fixes.** These are for scenarios where there could be multiple ways to fix an issue, or doing so would be unsafe. These require an explicit action to fix.

### Diagnostics

Diagnostics are what Rome calls errors. They are emitted absolutely everywhere when Rome finds a problem. This includes CLI argument parsing, JSON normalization, module resolution, lint errors, and more.

#### Anatomy

Diagnostics consist of six main parts:

- The header contains the **filename**, **line**, and **column**. They refer to the position that we believe is the root of an issue.
- Followed is the **message** which contains a summary of what we believe is wrong.
- The **Code frame** contains a snippet of the file referred in the header.
- **Advice** is freeform and appears at the end of a diagnostic. It can include additional messages, lists, other code frames, and more. It gives you more details about why you're seeing the diagnostic, and how you might fix it.

{% include docs/diagnostic-anatomy.md %}

Diagnostics that are fixable are indicated with a label that appears in the header:

{% include docs/diagnostic-anatomy-fixable.md %}

#### Reviewing Diagnostics

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

You can provide an additional explanation that includes why you suppressed the error in the first place, by prefixing it with a colon.

```javascript
// rome-ignore lint/js/noExplicitAny: Describe here why you are suppressing this error.
```

### Editor Integration

Get the most out of Rome by integrating it with your editor. You will get diagnostics as you type, and saving will automatically format your files.

Rome implements the [Language Server Protocol (LSP)](https://microsoft.github.io/language-server-protocol/) supported by [various editors](https://microsoft.github.io/language-server-protocol/implementors/tools/). We have official extensions available for:

- [VSCode](#)

Once an editor extension has been installed, the version of Rome in your project will be automatically found and used. As we improve Rome and add new functionality any changes will automatically work with your editor!

We welcome contributions adding official extensions for other mainstream editors. See [contributing](/contributing) for more information. LSP communication is done by [`rome lsp` command](#rome-lsp).
