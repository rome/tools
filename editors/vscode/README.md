# Minimal LSP client for development

## Preparing the LSP server executable

From the root of the repository:

```
cargo install --path crates/rome_lsp
```

Alternately, you can set `"rome.lspBin"` in your vscode config to point to the `rome_lsp` binary of your choice (e.g. a debug build).

## Installing the LSP client extension into VS Code

```
npm install
npm run build
npm run install-extension
```
