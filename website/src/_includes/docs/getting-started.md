## Getting Started

### Installation and quickstart

#### [Yarn](https://yarnpkg.com/):

```bash
yarn add rome
yarn rome init
```

#### [npm/npx](https://www.npmjs.com/):

```bash
npx rome init
```

After running this command, Rome:

- Iexecute a self-installation by adding itself to the `package.json` as dependency of your project (if there's a `package.json` already)
- Generates a project configuration which creates `.config` folder, with `rome.rjson` which serves as a project config.

If you're putting Rome into an already established project and you'd like to automatically apply formatting and fixes, you can use:

```bash
npx rome init --apply
```

Refer to [Project Configuration](#project-configuration) for configuration options.

> Note: The `.rjson` extension. [RJSON](#rome-json) is a superset of JSON that supports more-concise syntax and features such as comments.

### Start Linting

You can now run Rome commands! Linting can be accessed via the command:

```bash
npx rome check
```

Continue to the next section to learn more about linting in Rome!
