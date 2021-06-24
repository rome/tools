## Getting Started

### Installation and Usage

#### [Yarn](https://yarnpkg.com/):

```bash
yarn add rome
yarn rome init
```

#### [npm/npx](https://www.npmjs.com/):

```bash
npx rome init
```

After running this command, Rome will:

- Add itself to `package.json` as dependency if it wasn't present, and run your package manager to install
- Generate `.config/rome.json` that serves as your project config.

If you're putting Rome into an already established project and you'd like to automatically apply formatting and fixes, you can use:

```bash
npx rome init --apply
```

Refer to [Project Configuration](#project-configuration) for configuration options.

### Start Linting

You can now run Rome commands! Linting can be accessed via the command:

```bash
npx rome check
```

Continue to the next section to learn more about linting in Rome!
