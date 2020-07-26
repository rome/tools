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

A project is a folder and a configuration file. It tells Rome that the folder is your base directory and all files inside can be processed by Rome.

To automatically create a project, you can use the [`rome init`](/docs/cli/commands/init) command:

```bash
rome init
```

This will lint, autofix, and format all the files in the directory, and create a `rome.rjson` that contains your project config.

Refer to [Project Configuration](/docs/project-config) for configuration options.

> Note: The `.rjson` extension. [RJSON](/docs/rjson) is a superset of JSON that supports more-concise syntax and features such as comments.
