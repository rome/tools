## Getting Started

Rome works on Linux, macOS, and Windows.

For all operating systems, we support both x86_64 and ARM architectures.

Install Rome using any of the following methods.

### Install official Rome VS Code extension

You can use Rome by installing the [VS Code extension](https://marketplace.visualstudio.com/items?itemName=rome.rome) from the marketplace.

Rome currently doesn't support other editors than VS Code. [Let us know](https://github.com/rome/tools/discussions/categories/suggestions) if you would like support for another editor.

### Install Rome CLI

Install `rome` using your preferred node package manager. We require a minimum Node version of v14.18.

> NOTE: we recommend **not** installing the binary globally, instead please install the binary
> locally to your project. You can also use `npx`, `pnpm dlx` or `yarn dlx` to run `rome@next`.

| Package manager               | Instructions            |
| ----------------------------- | ----------------------- |
| [npm](https://www.npmjs.com/) | `npm i -D rome@next`    |
| [pnpm](https://pnpm.io/)      | `pnpm add -D rome@next` |
| [yarn](https://yarnpkg.com/)  | `yarn add -D rome@next` |

If you install the CLI locally, use the [scripts field](https://docs.npmjs.com/cli/v8/using-npm/scripts) of your package.json to run Rome. For instance:

```json
{
	"scripts": {
		"format": "rome format ."
	}
}
```

Then you can run:

```bash
npm run format
yarn format
pnpm format
```

### Install Rome in your CI pipeline

#### GitHub Actions

Please refer to the [`setup-rome` action documentation](https://github.com/rome/setup-rome#usage) for more information about its configuration:

```yaml
steps:
  - uses: rome/setup-rome@v0.1
    with:
      version: latest
  - run: rome --help
```

#### Installation on any other CI

You can download and install the binary directly using `curl`:

```shell
curl -L https://github.com/rome/tools/releases/download/latest/rome-<OS>-<ARCH> -o rome
chmod +x rome
```

Where `<OS>` and `<ARCH>` follow the Node.js syntax convention:

- `<OS>` is one of the following values: `win32`, `darwin` or `linux`
- `<ARCH>` is one of the following values: `arm64` or `x64`

> NOTE: For Windows Subsystem for Linux (WSL), please use `linux` as your OS

|         | `win32`         | `darwin`         | `linux`         |
| ------- | --------------- | ---------------- | --------------- |
| `arm64` | [`win32-arm64`] | [`darwin-arm64`] | [`linux-arm64`] |
| `x64`   | [`win32-x64`]   | [`darwin-x64`]   | [`linux-x64`]   |

Please make sure to choose the correct architecture from the [releases page](https://github.com/rome/tools/releases).

## Post-Installation

- check the [formatter section](/formatter#use-the-formatter-with-the-cli) for options and commands;
- check the options available in the [VS Code extension](/formatter#use-the-formatter-with-the-vscode-extension)

[`win32-arm64`]: https://github.com/rome/tools/releases/latest/download/rome-win32-arm64.exe
[`darwin-arm64`]: https://github.com/rome/tools/releases/latest/download/rome-darwin-arm64
[`linux-arm64`]: https://github.com/rome/tools/releases/latest/download/rome-linux-arm64
[`win32-x64`]: https://github.com/rome/tools/releases/latest/download/rome-win32-x64.exe
[`darwin-x64`]: https://github.com/rome/tools/releases/latest/download/rome-darwin-x64
[`linux-x64`]: https://github.com/rome/tools/releases/latest/download/rome-linux-x64
