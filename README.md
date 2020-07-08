<p align="center">
  <img alt="Rome, logo of an ancient Greek spartan helmet" src="https://github.com/romefrontend/rome/raw/main/assets/PNG/logo_transparent.png" width="700">
</p>

**Rome** is a linter, compiler, bundler, and [more](https://romefrontend.dev/#development-status) for JavaScript, TypeScript, HTML, Markdown, and CSS. Read more about our language support [here](https://romefrontend.dev/docs/language-support).

**Rome** unifies functionality that has previously been completely separate tools. Most frontend tooling have a significant overlap in responsibilities and implementation. There is value in these being a single tool. Building upon a shared base allows us to provide a cohesive experience for processing code, displaying errors, parallelizing work, caching, and configuration.

**Rome** has been built from scratch without the usage of existing libraries. **Rome** contains no third-party library dependencies.

**Rome** has strong conventions and aims to have minimal configuration. Read more about our project philosophy [here](https://romefrontend.dev/contributing/philosophy).

**Rome** is maintained by a [team of contributors](https://romefrontend.dev/contributing/team). **Rome** was started by [Sebastian McKenzie](https://twitter.com/sebmck), the author of [Babel](https://babeljs.io) and [Yarn](https://yarnpkg.com).

**Rome** is [MIT licensed](LICENSE), and the project managed under the [Contributor Covenant Code of Conduct](./CODE_OF_CONDUCT.md).

## History

**Rome** gets its name from proverbs such as "All Roads Lead to Rome", "Rome wasn't built in a day" and "When in Rome, do as the Romans do". This refers to the expansive scope and the desire for conformity across the project. It started as a joke at the office.

**Rome** has a logo of a Roman arch, one of the most influential patterns in architecture. It symbolizes a strong foundation, allowing you to build large projects without having to ponder the underlying architecture, and reinventing the wheel.

## Codebase

**Rome** is written completely in TypeScript with sparing usage of loose types.

**Rome** is a monorepo with [internal packages](packages/@romefrontend) to delineate code boundaries.

**Rome** is [self-hosted](<https://en.wikipedia.org/wiki/Self-hosting_(compilers)>) and compiles itself with an old version.

**Rome** supports processing [JSX](https://reactjs.org/docs/introducing-jsx.html) and [TypeScript](https://www.typescriptlang.org/) annotated code.

See [CONTRIBUTING](./CONTRIBUTING.md) for more information.

## Status

The current area of focus is **linting**. See the umbrella task [#20](https://github.com/romefrontend/rome/issues/20) for tracking.

## Getting Started

To setup Rome in a project, all you need is a `rome.json` file.

```bash
$ mkdir hello-world
$ cd hello-world
$ rome init
```

This file is used to configure Rome and indicates the boundaries of your project.

See [Getting Started](https://romefrontend.dev/docs/introduction/getting-started/) for more usage instructions.

## Philosophy

Project philosophy can be found on our [website](https://romefrontend.dev/contributing/philosophy).

## Community

Contribution and development instructions can be found in [CONTRIBUTING](./CONTRIBUTING.md).

Additional project coordination and realtime discussion happens on our [Discord server](https://discord.gg/9WxHa5d). Remember that all activity on the Discord is still moderated and will be strictly enforced under the project's [Code of Conduct](./CODE_OF_CONDUCT.md).
