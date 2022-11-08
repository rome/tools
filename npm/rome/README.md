<p align="center">
	<picture>
		<source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/rome/brand/main/PNG/logo_white_yellow_transparent.png" width="700">
		<img alt="Rome's logo depicting an ancient Roman arch with the word Rome to its side" src="https://raw.githubusercontent.com/rome/brand/main/PNG/logo_transparent.png" width="700">
	</picture>
</p>

<div align="center">

[![MIT licensed][mit-badge]][mit-url]
[![Discord chat][discord-badge]][discord-url]
[![CI on main][ci-badge]][ci-url]
[![npm version][npm-badge]][npm-url]
[![VSCode version][vscode-badge]][vscode-url]


[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg?color=brightgreen
[mit-url]: LICENSE
[discord-badge]: https://img.shields.io/discord/678763474494423051?logo=discord&label=discord&color=brightgreen
[discord-url]: https://discord.gg/rome
[ci-badge]: https://github.com/rome/tools/actions/workflows/main.yml/badge.svg
[ci-url]: https://github.com/rome/tools/actions/workflows/main.yml
[npm-badge]: https://img.shields.io/npm/v/rome/next?color=brightgreen
[npm-url]: https://www.npmjs.com/package/rome/v/next
[vscode-badge]: https://img.shields.io/visual-studio-marketplace/v/rome.rome?color=brightgreen&label=vscode
[vscode-url]: (https://marketplace.visualstudio.com/items?itemName=rome.rome

</div>

**Rome** is a formatter, linter, bundler, and [more](https://rome.tools/#development-status) for JavaScript, TypeScript, JSON, HTML, Markdown, and CSS.

**Rome** is designed to replace [Babel](https://babeljs.io/), [ESLint](https://eslint.org/), [webpack](https://webpack.js.org/), [Prettier](https://prettier.io/), [Jest](https://jestjs.io/), and others.

**Rome** unifies functionality that has previously been separate tools. Building upon a shared base allows us to provide a cohesive experience for processing code, displaying errors, parallelizing work, caching, and configuration.

**Rome** has strong conventions and aims to have minimal configuration. Read more about our [project philosophy](https://rome.tools/#philosophy).

**Rome** is [written in Rust](https://rome.tools/blog/2021/09/21/rome-will-be-rewritten-in-rust).

**Rome** has first-class IDE support, with a sophisticated parser that represents the source text in full fidelity
and top-notch error recovery.

**Rome** is [MIT licensed](https://github.com/rome/tools/tree/main/LICENSE) and moderated under the [Contributor Covenant Code of Conduct](https://github.com/rome/tools/tree/main/CODE_OF_CONDUCT.md).


## Installation

```shell
npm i rome@next
```

## Usage

Format files:

```shell
rome format --write ./path ./path/to/file.js
```

For complete documentation, please visit the [official website].


## Philosophy

The project philosophy can be found on our [website](https://rome.tools/#philosophy).

## Community

Contribution and development instructions can be found in [CONTRIBUTING](./CONTRIBUTING.md).

Additional project coordination and real-time discussion happens on our [Discord server](https://discord.gg/rome). Remember that all activity on the Discord server is still moderated and will be strictly enforced under the project's [Code of Conduct](./CODE_OF_CONDUCT.md).

## Technical documentation

If you're curious to know our internals, you can browse our [technical documentation].

[official website]: https://rome.tools/
[technical documentation]: https://rome.github.io/tools/rome

