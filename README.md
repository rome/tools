<p align="center">
  <img alt="Rome, logo of an ancient Greek spartan helmet" src="https://github.com/romejs/rome/raw/main/assets/PNG/logo_transparent.png" width="700">
</p>

**Rome** is an experimental JavaScript toolchain. It includes a compiler, linter, formatter, bundler, testing framework and more. It aims to be a comprehensive tool for anything related to the processing of JavaScript source code.

**Rome** is not a collection of existing tools. All components are custom and use no third-party dependencies.

**Rome** is experimental and in active development. It's open for contributors and those interested in experimental tools. **It is not ready for production usage. The only way to use it is to build from source.**

**Rome** aims to be a replacement for many existing JavaScript tools. We will, however, offer integrations for components in other tools. For example, using the Rome compiler as a plugin for another bundler.

**Rome** is [MIT licensed](LICENSE), and the project managed under the [Contributor Covenant Code of Conduct](./CODE_OF_CONDUCT.md).

## History

**Rome** was started by [Sebastian McKenzie](https://twitter.com/sebmck), the author of [Babel](https://babeljs.io) and [Yarn](https://yarnpkg.com).

**Rome** gets its name from proverbs such as "All Roads Lead to Rome", "Rome wasn't built in a day" and "When in Rome, do as the Romans do". This refers to the expansive scope and the desire for conformity across the project. It started as a joke at the office.

**Rome** has a logo of a Roman arch, one of the most influential patterns in architecture. It symbolizes a strong foundation, allowing you to build large projects without having to ponder the underlying architecture, and reinventing the wheel.

## Codebase

**Rome** is written completely in TypeScript with sparing usage of loose types.

**Rome** is a monorepo with [internal packages](packages/@romejs) to delineate code boundaries.

**Rome** is [self-hosted](<https://en.wikipedia.org/wiki/Self-hosting_(compilers)>) and compiles itself with an old version.

**Rome** supports processing [JSX](https://reactjs.org/docs/introducing-jsx.html) and [TypeScript](https://www.typescriptlang.org/) annotated code.

See [CONTRIBUTING](./CONTRIBUTING.md) for more information.

## Status

The current area of focus is **linting**. See the umbrella task [#20](https://github.com/romejs/rome/issues/20) for tracking.

## Getting Started

To setup Rome in a project, all you need is a `rome.json` file.

```bash
$ mkdir hello-world
$ cd hello-world
$ rome init
```

This file is used to configure Rome and indicates the boundaries of your project.

See [Getting Started](https://romejs.dev/docs/introduction/getting-started/) for more usage instructions.

## Philosophy

Project philosophy can be found on our [website](https://preview.romejs.dev/contributing/philosophy).

## Community

Contribution and development instructions can be found in [CONTRIBUTING](./CONTRIBUTING.md).

Additional project coordination and realtime discussion happens on our [Discord server](https://discord.gg/9WxHa5d). Remember that all activity on the Discord is still moderated and will be strictly enforced under the project's [Code of Conduct](./CODE_OF_CONDUCT.md).
