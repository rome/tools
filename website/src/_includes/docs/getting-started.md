## Getting Started

Install Rome via one of the following alternatives.

### Installation of the CLI

You can install the `rome` binary via any node package manager.

> WARNING: we strongly suggest to **not** install the binary globally, instead install the binary
> locally to your project or use `npx/pnpx/yarn dlx` in order to leverage the CLI.

#### [Installation via Yarn](https://yarnpkg.com/)

```bash
yarn add rome@next
```

#### [Installation via npm](https://www.npmjs.com/)

```bash
npm i rome@next
```

#### [Installation via pnpm](https://pnpm.io/)

```bash
pnpm i rome@next
```

### Installation VS Code extension

You can use Rome by downloading the [VS Code extension](https://marketplace.visualstudio.com/items?itemName=rome.rome) from the marketplace. 


### Installation via CI

You can install Rome in a continues integration too.

#### Installation on GitHub

Please refer to the [official page](https://github.com/rome/setup-rome#usage) for more information about its configuration: 

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
curl -L https://github.com/rome/tools/releases/download/latest/rome-linux-x64 -o rome
chmod +x rome
```

Please make sure to choose the correct architecture from the [releases page](https://github.com/rome/tools/releases).


## After installation

- check the [formatter section](/formatter#use-the-formatter-with-the-cli) for options and commands;
- check the options available in the [VS Code extension](/formatter#use-the-formatter-with-the-vscode-extension)