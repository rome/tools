## Project Configuration

**Rome** needs to know how to find your project and what files it includes. To do this we require a project configuration file.

Your configuration can be placed in a [few different locations](#supported-locations), but we recommend using a single `rome.rjson` file. This file is written using [RJSON](#rome-json) which is our flavor of JSON. It supports comments and has a simpler syntax.

All properties are **optional**, you can even have an empty config! We recommend using the [`rome config`](#rome-config) command to modify your configuration, this works with any of the supported config locations, and when editing RJSON will even retain comments.

We are deliberately lean with the supported configuration. We do not include options just for the sake of personalization. We aim to offer everything out of the box and only introduce configuration if absolutely necessary.

```json
name: "project-name"
version: "^0.0.0"
root: true
extends: "../other-file"

lint: {
	ignore: []
	globals: []
}
dependencies: {
	exceptions: {
		invalidLicenses: {
			"funky-licence": ["lib@1.0.0", "lib@1.1.0", "other-lib@2.0.0"]
		}
	}
}
```

### Properties

#### `name`

This is your project name. It is typically whatever you have set as `name` in `package.json`. This is never shown to you, and is used internally to refer to your project.

The Rome cache is portable, meaning it contains no references to absolute paths. This allows it to be stored across different machines. This feature may not be important to you so it can be safely omitted in most cases.

```bash
rome config set name "project-name"
```

#### `extends`

Inherit from another file and merge configuration. If you would only like to share partial configuration then extract it into a separate config that is used instead.

If the file refers to a `package.json` file then the `rome` property is used.

```bash
rome config set-directory extends "some-other-file"
```

#### `root`

By default, Rome supports [nested projects](#nested-projects) and will search parent directories for other projects to initialize. Sometimes this isn't what you want and can cause unexpected problems. This can be solved by explicitly setting the `root` flag which tells Rome that it should ignore any parent directories.

```bash
rome config enable root
rome config disable root
```

#### `version`

This is a semver range of the Rome version you want to set your project to. It is an optional layer of protection and can avoid version mismatches in large monorepos and projects.

```bash
rome config set version "^0.0.0"
```

#### `lint.enabled`

Disables linting.

```bash
rome config push lint.enabled false
```

#### `lint.ignore`

[Path patterns](#path-patterns) that you want to ignore from linting.

```bash
rome config push lint.ignore "some-path"
```

#### `lint.globals`

Custom variables you want to declare as global.

```bash
rome config push lint.globals SomeGlobal
```

#### `lint.requireSuppressionExplanations`

Raise a diagnostic if a suppression does not have a [valid explanation](#explanation).

```bash
rome config enable lint.requireSuppressionExplanations
```

#### `lint.rules`

Allows users to control rules and categories


##### `lint.rules.recommended`

Enables only the rules recommended by Rome.

```bash
rome config enable lint.rules.recommended
```

##### `lint.rules.{category}.{rule}`

Allows turning on and off particular rule categories and particular rules.

The list of rule categories can be found in the [lint page](/docs/lint/rules/).

Below, there are some examples of how to control particular categories of rules.

> The key `recommended` can't be provided in conjunction with the specific categorie

```json
{
	"lint": {
		"rules": {
			"recommended": true
		}
	}
}
```

Disable all `a11y` rules and enable all `react` rules.

```json
{
	"lint": {
		"rules": {
			"a11y": false,
			"react": true
		}
	}
}
```

Disable all `a11y` rules and enable only the *recommended* `react` rules.

```json
{
	"lint": {
		"rules": {
			"a11y": false,
			"react": {
				"recommended": true
			}
		}
	}
}
```

Disable all `a11y` rules and enable all the `react` rules, exception for `noArrayIndexKey`,
which will be disabled.

```json
{
	"lint": {
		"rules": {
			"a11y": false,
			"react": {
				"noArrayIndexKey": false
			}
		}
	}
}
```

### `dependencies.exceptions`

Exception rules for your dependencies that don't pass validation.

#### `dependencies.exceptions.invalidLicenses`

Sometimes Rome might complain that one or more of your dependencies has an invalid license.

Optionally, you can insert the name of this invalid license here:

```bash
rome config push dependencies.exceptions.invalidLicenses.invalid-license-name "third-party-lib@0.1.0"
```

If you are unsure about the license name of your library, rome will suggest the command for
you when you try to run a command.

### Supported Locations

You can specify your project config in a few different places.

##### `.config/rome.rjson` (recommended)

This is the recommend location. It's the file we create when running `rome init`.

It can contains Rome's flavor of JSON, [RJSON](#rome-json), that allows comments and simpler syntax.

##### `.config/rome.json`

You can also use `rome.json` with regular JSON. This is useful if you think you might want to process and manipulate project configuration with another tool or language.

##### `package.json` field

Alternatively, your project config can be included in a `rome` field inside of `package.json`:

```json
{
	"name": "my-package",
	"version": "0.0.0",
	"rome": {
		"version": "^0.0.1"
	}
}
```

### Nested Projects

Nested projects are a first-class feature and can be used to customize configuration for different parts of your codebase. Multiple projects can be loaded inside of a single Rome process.

When running any command or operation on a file, we refer to the project it is a part of when considering any configuration rather than what directory it was ran from.

### Path Patterns

Some configuration options contain path patterns. If you have ever used `.gitignore` then it's the same familiar syntax. These are also called [glob patterns](https://en.wikipedia.org/wiki/Glob_(programming)).

These are paths that contain special characters for matching files and directories. These are typically used for ignore rules.

We support the following syntax:

##### Wildcards

`*` matches any number of any characters including none in a directory. This can be used in any filename position. ie.

```
*.js
App*Page.ts
```

##### Matching Directories

A pattern that matches a directory will also match every file inside of it. eg. `pages` is the same as writing `pages/**/*`.

##### Negations

Sometimes you want to add exceptions to a rule. For example, you have a folder you want to ignore but there is a file inside of that you don't want to match. You can do this by prefixing it with `!`. For example:

```text
scripts
!scripts/navigation.js
```

This will ignore everything in the `scripts` directory besides the file `navigation.js`.

##### Base Directory

Say that you have the following directory structure:

```text
babies/juniper
cats/babies/orion
cats/babies/nev
```

And you only wanted to ignore the folder `babies` that contains `juniper`. If you wrote just `babies` then it would match both directories. However, if you prefix it with a back slash, as in `/babies`, then it will only match the folder at the base.
