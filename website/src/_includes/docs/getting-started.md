## Getting Started

### Installation

Rome is available for installation via [Yarn](https://yarnpkg.com/):

```bash
yarn add rome
```

or [npm](https://www.npmjs.com/):

```bash
npm install rome
```

### Creating a Project

In order for Rome to find your files it needs a project configuration. To automatically create a project, you can use the [`rome init`](#rome-init) command:

```bash
rome init
```

This will create a `.config` directory and place a `rome.rjson` inside of it that contains your project config.

If you're putting Rome into an already established project and you'd like to automatically apply formatting and fixes, you can use:

```bash
rome init --apply
```

Refer to [Project Configuration](#project-configuration) for configuration options.

> Note: The `.rjson` extension. [RJSON](#rome-json) is a superset of JSON that supports more-concise syntax and features such as comments.

### Start Linting

You can now run Rome commands! Linting can be accessed via the command:

```bash
rome check
```

Continue to the next section to learn more about linting in Rome!
