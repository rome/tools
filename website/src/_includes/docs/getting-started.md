## Getting Started

Install Rome using any of the following installation methods.

### Install Rome official VS Code extension

You can use Rome by installing the [VS Code extension](https://marketplace.visualstudio.com/items?itemName=rome.rome) from the marketplace. 

Rome currently doesn't support other editors than VS Code. Let us know if you're interested in getting support for another editor to help us prioritize our work.

### Install Rome CLI

Install `rome` using your preferred node package manager.

> WARNING: we strongly suggest to **not** install the binary globally, instead install the binary
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

Please make sure to choose the correct architecture from the [releases page](https://github.com/rome/tools/releases).


## After installation

- check the [formatter section](/formatter#use-the-formatter-with-the-cli) for options and commands;
- check the options available in the [VS Code extension](/formatter#use-the-formatter-with-the-vscode-extension)