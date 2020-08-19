<p align="center">
	<img alt="Rome's logo depicting an ancient Roman arch with the word Rome to its side" src="https://github.com/romefrontend/rome/raw/main/assets/PNG/logo_transparent.png" width="700">
</p>

<!-- This intro is synced with the website via the `./rome run scripts/generate-files/website-intro` script. Make sure you run it after modifying anything between these comments. -->
<!-- INTRO START -->
**Rome** is a linter, compiler, bundler, and [more](https://romefrontend.dev/#development-status) for JavaScript, TypeScript, JSON, HTML, Markdown, and CSS.

**Rome** is designed to replace [Babel](https://babeljs.io/), [ESLint](https://eslint.org/), [webpack](https://webpack.js.org/), [Prettier](https://prettier.io/), [Jest](https://jestjs.io/), and others.

**Rome** unifies functionality that has previously been separate tools. Building upon a shared base allows us to provide a cohesive experience for processing code, displaying errors, parallelizing work, caching, and configuration.

**Rome** has strong conventions and aims to have minimal configuration. Read more about our [project philosophy](https://romefrontend.dev/#philosophy).

**Rome** is written in [TypeScript](https://www.typescriptlang.org/) and runs on [Node.js](https://nodejs.org/en/). **Rome** has zero dependencies, and has largely been written from scratch. See [credits](https://romefrontend.dev/credits) for more information.

**Rome** is maintained by a [team of volunteers](https://romefrontend.dev/credits#team) under an established [governance model](https://github.com/romefrontend/rome/blob/main/GOVERNANCE.md).

**Rome** is [MIT licensed](https://github.com/romefrontend/rome/tree/main/LICENSE) and moderated under the [Contributor Covenant Code of Conduct](https://github.com/romefrontend/rome/tree/main/CODE_OF_CONDUCT.md).
<!-- INTRO END -->

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

See [Getting Started](https://romefrontend.dev/#getting-started) guide for more usage instructions.

## Philosophy

The project philosophy can be found on our [website](https://romefrontend.dev/#philosophy).

## Community

Contribution and development instructions can be found in [CONTRIBUTING](./CONTRIBUTING.md).

Additional project coordination and real-time discussion happens on our [Discord server](https://discord.gg/9WxHa5d). Remember that all activity on the Discord server is still moderated and will be strictly enforced under the project's [Code of Conduct](./CODE_OF_CONDUCT.md).
