# Rome JavaScript Bindings

Official JavaScript bindings for the package [rome](https://www.npmjs.com/package/rome)

> **Warning**:
> The API is currently in alpha. It is not yet ready for production use. We appreciate your support and feedback as we work to make it ready for everyone.

## Installation

```shell
npm i rome
npm i @rometools/js-api
```

The package `rome` is marked as **peer dependency** of this package.

## Usage

```js
import { Rome, BackendKind } from "@rometools/js-api";

const rome = await Rome.create({
	backendKind: BackendKind.NODE,
});

const result = await rome.formatContent("function f   () {  }", {
	filePath: "example.js",
});

console.log(result.content);
```

## Philosophy

The project philosophy can be found on our [website](https://rome.tools/#philosophy).

## Community

Contribution and development instructions can be found in [CONTRIBUTING](./CONTRIBUTING.md).

Additional project coordination and real-time discussion happens on our [Discord server](https://discord.gg/rome). Remember that all activity on the Discord server is still moderated and will be strictly enforced under the project's [Code of Conduct](./CODE_OF_CONDUCT.md).
