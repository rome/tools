---
id: contributing
title: Contributing
sidebar_label: Contributing
---

We can use help in a bunch of areas and any help is appreciated. Our [GitHub issues](https://github.com/facebookexperimental/rome/issues) serve as a place for any discussion, whether it's bug reports, questions, project direction etc. As the project grows this policy may change.

Our [Discord server](https://discord.gg/9WxHa5d) is open for help and more adhoc discussion. All activity on the Discord is still moderated and will be strictly enforced under the project's [Code of Conduct](https://github.com/facebookexperimental/rome/blob/master/.github/CODE_OF_CONDUCT.md).

## Getting Started

Getting started with developing Rome is as easy as three commands. You will need Node v12 or above.

```
git clone https://github.com/facebookexperimental/rome
cd rome
scripts/dev-rome --help
```

No dependency installation step is required as we check in our `node_modules` folder that contains only a copy of TypeScript and some definitions.

Refer to [Getting Started](../introduction/getting-started.md) for more usage documentation.

## Testing

You can run the test suite with the following command:

```
scripts/dev-rome test
```

This will run all tests inside of any `__rtests__` directories.

## Type Checking

Run TypeScript with code emitting disabled to perform a full typecheck outside the editor.

```
node_modules/.bin/tsc --noEmit
```
