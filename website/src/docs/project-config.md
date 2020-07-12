---
title: Project Configuration
layout: layouts/page.njk
---

# Project Configuration

## Fields

### `name`

```text
name: "project-name"
```

This can also be configured via the [`rome config` command](/docs/cli/commands/config):

```bash
rome config set name "project-name"
```

### `version`

```text
version: "^0.0.0"
```

This can also be configured via the [`rome config` command](/docs/cli/commands/config):

```bash
rome config set version "^0.0.0"
```

### `check`

See [Linting](/docs/lint#project-config) for lint config options.

## Supported Locations

You can specify your project config in a few different places.

### `rome.rjson` (recommended)

This is the recommend location. It's the file we create when running `rome init`.

It can contains Rome's flavor of JSON, [RJSON](/docs/rjson), that allows comments and omitting syntax.

### `rome.json`

You can also use `rome.json` that does not allow Rome JSON extensions. This is useful if you think you might want to process and manipulate project configuration with another tool.

### `package.json` field

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

## Path patterns

- `foo.*`
- `/foo`
- `foo/bar`
- `foo/**/bar`
- `!foo`
