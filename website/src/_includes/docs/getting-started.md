## Getting Started

Rome works on Linux, macOS, and Windows operative systems. 

On macOS, Linux and Windows we both support x64 and ARM architectures.

Install Rome using any of the following installation methods.

### Install Rome official VS Code extension

You can use Rome by installing the [VS Code extension](https://marketplace.visualstudio.com/items?itemName=rome.rome) from the marketplace. 

Rome currently doesn't support other editors than VS Code. Let us know if you're interested in getting support for another editor to help us prioritize our work.

### Install Rome CLI

Install `rome` using your preferred node package manager.

> RECOMMENDATION: we advise to **not** install the binary globally, instead install the binary
> locally to your project or use `npx/pnpx/yarn dlx` in order to leverage the CLI.


| Package manager               | Instructions         |
|-------------------------------|----------------------|
| [npm](https://www.npmjs.com/) | `npm i rome@next`    |
| [pnpm](https://pnpm.io/)      | `pnpm i rome@next`   |
| [yarn](https://yarnpkg.com/)  | `yarn add rome@next` |


### Install Rome in your CI pipeline

#### GitHub Actions

Please refer to the documentation of the [`setup-rome` action](https://github.com/rome/setup-rome#usage) for more information about its configuration: 

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

Where `OS` and `ARCH` follow the Node.js syntax convention:

- `OS` is one of the following values: `win32`, `darwin` or `linux`
- `ARCH` is one of the following values: `arm64` or `x64`


|         | `win32`         | `darwin`         | `linux`         |
|---------|-----------------|------------------|-----------------|
| `arm64` | [`win32-arm64`] | [`darwin-arm64`] | [`linux-arm64`] | 
| `x64`   | [`win32-x64`]   | [`darwin-x64`]   | [`linux-x64`]   | 

Please make sure to choose the correct architecture from the [releases page](https://github.com/rome/tools/releases).


## After installation

- check the [formatter section](/formatter#use-the-formatter-with-the-cli) for options and commands;
- check the options available in the [VS Code extension](/formatter#use-the-formatter-with-the-vscode-extension)


[`win32-arm64`]: https://github.com/rome/tools/releases/latest/download/rome-win32-arm64.exe
[`darwin-arm64`]: https://github.com/rome/tools/releases/latest/download/rome-darwin-arm64
[`linux-arm64`]: https://github.com/rome/tools/releases/latest/download/rome-linux-arm64
[`win32-x64`]: https://github.com/rome/tools/releases/latest/download/rome-win32-x64.exe
[`darwin-x64`]: https://github.com/rome/tools/releases/latest/download/rome-darwin-x64
[`linux-x64`]: https://github.com/rome/tools/releases/latest/download/rome-linux-x64