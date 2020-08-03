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

This will create a `rome.rjson` that contains your project config. This will also lint, autofix, and format all the files in the directory.

Refer to [Project Configuration](#project-config) for configuration options.

> Note: The `.rjson` extension. [RJSON](#rome-json) is a superset of JSON that supports more-concise syntax and features such as comments.

### Start Linting

You can now run Rome commands! Linting can be accessed via the command:

```bash
rome check
```

Continue to the next section to learn more about linting in Rome!
