# Contributing

We can use help in a bunch of areas and any help is appreciated. Our [GitHub issues](https://github.com/romejs/rome/issues) serve as a place for any discussion, whether it's bug reports, questions, project direction etc. As the project grows this policy may change.

Our [Discord server](https://discord.gg/9WxHa5d) is open for help and more adhoc discussion. All activity on the Discord is still moderated and will be strictly enforced under the project's [Code of Conduct](./CODE_OF_CONDUCT.md).

You have to sign the [contributor license agreement (CLA)](https://code.facebook.com/cla) before your pull request can be reviewed.

## Getting Started

Getting started with developing Rome is as easy as three commands. You will need Node v12 or above.

```bash
git clone https://github.com/romejs/rome
cd rome
./scripts/dev-rome --help
```

**Note:** If you previously ran the user-facing [installation instructions](https://romejs.dev/docs/introduction/installation), the `dist` directory must be deleted before running any development commands.

No dependency installation step is required as we check in our `node_modules` folder that contains only a copy of TypeScript and some definitions.

Refer to [Getting Started](https://romejs.dev/docs/introduction/getting-started/) for more usage documentation.

## Testing

You can run the test suite with the following command:

```bash
./scripts/dev-rome test
```

This will run all tests inside of any `__rtests__` directories.

## Type Checking

Run TypeScript with code emitting disabled to perform a full typecheck outside the editor.

```bash
node_modules/.bin/tsc --noEmit
```

## Developing on Windows

You may run into errors when trying to run the Rome commands on Windows

```
PS C:\code\rome> scripts/dev-rome --help
ResourceUnavailable: Program 'dev-rome' failed to run: No application is associated with the specified file for this operation.At line:1 char:1
+ scripts/dev-rome --help
+ ~~~~~~~~~~~~~~~~~~~~~~~.
```

This is because the command uses shebangs to automatically invoke itself as a Node script. You can fix this in a couple of ways:

- Use a terminal that supports shebangs on Windows such a Git Bash
- Prefix any commands with `node` eg. `node scripts/dev-rome`
