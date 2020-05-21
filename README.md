<p align="center">
  <img alt="Rome, logo of an ancient Greek spartan helmet" src="https://github.com/romejs/logo/raw/master/PNG/logo_transparent.png" width="700">
</p>

**Rome** is an experimental JavaScript toolchain. It includes a compiler, linter, formatter, bundler, testing framework and more. It aims to be a comprehensive tool for anything related to the processing of JavaScript source code.

**Rome** is not a collection of existing tools. All components are custom and use no third-party dependencies.

**Rome** is experimental and in active development. It's open for contributors and those interested in experimental tools. **It is not ready for production usage. The only way to use it is to build from source.**

**Rome** aims to be a replacement for many existing JavaScript tools. We will, however, offer integrations for components in other tools. For example, using the Rome compiler as a plugin for another bundler.

**Rome** is [MIT licensed](LICENSE), and the project managed under the [Contributor Covenant Code of Conduct](.github/CODE_OF_CONDUCT.md).

## History

**Rome** was started by [Sebastian McKenzie](https://twitter.com/sebmck), the author of [Babel](https://babeljs.io) and [Yarn](https://yarnpkg.com).

**Rome** gets its name from proverbs such as "All Roads Lead to Rome", "Rome wasn't built in a day" and "When in Rome, do as the Romans do". This refers to the expansive scope and the desire for conformity across the project. It started as a joke at the office.

**Rome** has a logo of a Roman arch, one of the most influential patterns in architecture. It symbolizes a strong foundation, allowing you to build large projects without having to ponder the underlying architecture, and reinventing the wheel.

## Codebase

**Rome** is written completely in TypeScript with sparing usage of loose types.

**Rome** is a monorepo with [internal packages](packages/@romejs) to delineate code boundaries.

**Rome** is [self-hosted](<https://en.wikipedia.org/wiki/Self-hosting_(compilers)>) and compiles itself with an old version.

**Rome** supports processing [JSX](https://reactjs.org/docs/introducing-jsx.html) and [TypeScript](https://www.typescriptlang.org/) annotated code.

See [CONTRIBUTING](.github/CONTRIBUTING.md) for more information.

## Status

The current area of focus is **linting**. See the umbrella task [#20](https://github.com/romejs/rome/issues/20) for tracking.

## Getting Started

To setup Rome in a project, all you need is a `rome.json` file.

```bash
$ mkdir hello-world
$ cd hello-world
$ echo '{}' >rome.json
```

This file is used to configure Rome and indicates the boundaries of your project.

See [Getting Started](https://romejs.dev/docs/introduction/getting-started/) for more usage instructions.

## Philosophy

This list includes general ethos the project should abide by. This list is not comprehensive. Some of these are obvious but are stated for completeness.

### Project Management

- **Set clear expectations.** Make project intent and decisions known well in advance. Nothing should be a surprise.
- **Transparency.** No back-channel project management. Project conversation and decisions will take place only on public forums such as GitHub, the Rome Discord, and Twitter. The only exception to this is moderation decisions which will be strictly done in private.

### Technical

- **No external dependencies.** This allows us to develop faster and provide a more cohesive experience by integrating internal libraries more tightly and sharing concepts and abstractions. There always exist opportunities to have a better experience by having something purpose-built.
- **Errors should suggest fixes and hints where possible.** These should be inferred and filtered from usage to reduce surfacing irrelevant and unhelpful messages.
- **Unique and specific error messages.** No generic error messages. This not only helps users understand what went wrong, but should provide maintainers with a unique call site and the necessary information to debug.
- **Minimize API.** Question the existence of all options and flags. Are they necessary? Can they be combined? How can we reduce code branching?
- **Reduce jargon.** Don't assume that users will understand specific terminology. Strive to provide clear meaning for experts and beginners. For example, use "character" where you would traditionally use "token" when producing parser errors.
- **Utilize verbosity when naming commands and flags.** No unnecessary and confusing abbreviations.
- **Use inclusive terminology.** Use gender-neutral pronouns. No ableist slurs. No usage of terms that could be considered insensitive.
- **Build for generic clients.** Don't assume that output will only be consumed by a terminal and using ANSI codes. Use abstractions that could be generalized for viewing in an IDE, browser, or other environments.
- **Use strong types.** Don't use loose types such as `any`. Where possible, refine and validate input. Aim for sound types.
- **Terminal output should be unambiguous.** When designing terminal output, don't purely rely on formatting cues such as color. Always use a combination of formatting, symbols, and spacing. If all ANSI codes are stripped, all the output should still be understood.

## Community

Contribution and development instructions can be found in [CONTRIBUTING](.github/CONTRIBUTING.md).

Additional project coordination and realtime discussion happens on our [Discord server](https://discord.gg/9WxHa5d). Remember that all activity on the Discord is still moderated and will be strictly enforced under the project's [Code of Conduct](CODE_OF_CONDUCT.md).
