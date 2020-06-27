---
title: Project Configuration
layout: layouts/base.njk
---

# Project Configuration

## Fields

### `name`

```bash
rome config set name "project-name"
```

### `version`

```bash
rome config set version "^0.0.0"
```

### `lint`

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
