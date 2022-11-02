# Rome VS Code Extension

Adds support for the Rome language server to provide formatting, diagnostics, and code actions.

## How the extension works

The extension acts as a language server, which means that you will get all the features
provided by the extension (formatting, diagnostics, etc.) for any file supported by Rome.

The extension automatically discovers the `rome.json` file in the workspace root directory.

## Known limitations

There are a few limitations that the team plans to remove in the following releases.

- the discovery of the `rome.json` is limited to the root of the workspace
- changes to the configuration `rome.json` won't be picked automatically; use the VSCode command
  `Restart LSP Server` or use `rome stop` command;
- when updating `rome` inside you `package.json`, the rome server needs to be restarted;
  you can use `rome stop` command or kill the process manually;
- if you don't want to see Rome's diagnostics in projects that don't use Rome, you need to disable
  the extension;

## Supported languages

Check the [website](https://rome.tools/#language-support) for more information.

## Usage

This extension may be bundled with a prebuilt binary for `rome_lsp`, the Rome language server.

You can set the path to a `rome_lsp` executable using the `"rome.lspBin"` setting in your VS Code Settings.
