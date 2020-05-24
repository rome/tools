# Installation

Rome requires you to have `node` and `npm` installed on your system. Make sure you have these before you proceed.

Rome is currently not available via `npm`, and must be installed from GitHub. In a folder of your choice, clone the `rome` repository.

```bash
git clone https://github.com/romejs/rome
```

Then, navigate into it and build `rome`:

```bash
cd rome; ./scripts/build-release dist
```

On Windows 10, build `rome` using the following command in PowerShell 7:

```bash
cd rome && node scripts/build-release dist
```

As a final step, install `rome` globally:

```bash
npm install -g ./dist/
```

Congratulations! Rome is installed.

When it comes time to update Rome, repeat the above process. `npm` will automatically overwrite your existing Rome installation with the new version.
