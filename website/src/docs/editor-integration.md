---
title: Editor Integration
layout: layouts/base.njk
---

# Editor Integration

Rome implements the [Language Server Protocol (LSP)](https://microsoft.github.io/language-server-protocol/) implemented by [various editors](https://microsoft.github.io/language-server-protocol/implementors/tools/).

When an editor extension is installed, it will find and start the version of Rome installed into your project. As we improve Rome any changes will automatically work with your editor!

## VSCode

TODO

## Others

We welcome contributions adding official extensions for other mainstream editors. See [contributing](#contributing) for more information!

Implementation details:

 - The `rome` location can be found by traversing up the projects directory chain and looking for `node_modules/rome/bin/rome/index.js`.
 - A LSP connection can be established by running `rome lsp` and communicating over stdio. 
