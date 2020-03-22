---
id: installation
title: Installation
sidebar_label: Installation
---

## Before you continue

To install Rome, you must have `node` and `npm` installed on your system. If you do not have `node` and `npm`
installed, install them before continuing.

## Cloning and building

Rome is not available via `npm` and must be installed from GitHub.
In a folder of your choice, clone the `rome` repository:

```bash
git clone https://github.com/facebookexperimental/rome
```

Then, navigate into it and build `rome`:

```bash
cd rome; ./scripts/build-release dist
```

Now, install `rome` globally:

```
npm install -g ./dist/
```

Congratulations! Rome is installed.

When it comes time to update Rome, repeat the above process. `npm` will
automatically overwrite your existing Rome installation with the new version.