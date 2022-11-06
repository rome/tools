---
title: CLI
---

# CLI

## Commands

### `rome init`

Helps you to set up Rome for a new project by guiding you through the creation of a new `rome.json` [configuration](/configuration) file.

The command fails if the project already has a `rome.json` configuration file.

### `rome version`

Prints the version of the CLI and whether there's a server (daemon) connected

### `rome rage`

Prints information for debugging purpose

### `rome lsp-proxy`

It starts a server for the LSP protocol, which communicates over `stdin` and `stdout`.

This command is useful to interact with the Rome server when developing editors/IDEs. 

### `rome format`

Runs the formatter on a set of files.

### `rome check`

Runs the linter on a set of files and reports errors and warnings to the console.

### `rome ci`

Runs the linter and verifies the formatting of a set of files. It reports errors to the console. If any errors are found the process exits with a code of `1`.

This command is intended to be used in CI workflows.

### `rome start`

Start the Rome [daemon](/internals/architecture#daemon) server

### `rome stop`

Stop the Rome [daemon](/internals/architecture#deamon) server

## Common Options

### `--no-colors`

Disable the formatting of markup (print everything as plain text)

### `--use-server`

Connect to a running instance of the Rome daemon server

##### `--files-max-size`

The maximum allowed size for source code files in bytes.

> Default: 1024

## Global Options

Use these flags to get information about the Rome CLI.

### `--version`

Prints the Rome version and exits.

### `--help`

Prints the help message and exits.
