---
layout: layouts/docs.liquid
title: Configuration
---

<!-- Make sure to update the redirect in `static/_redirects` when changing the configuration title -->

# Configuration

The configuration file is considered **optional**, Rome has good defaults. Use the configuration
file to change those defaults.

The Rome configuration file is named `rome.json` and should be placed in the root directory of your project. The root
directory is usually the directory containing your project's `package.json`.

Here's an example:

```json
{
  "formatter": {
    "enabled": true,
    "indentStyle": "tab",
    "lineWidth": 120
  },
  "linter": {
    "enabled": false
  }
}
```

This configuration file enables the linter and formatter and sets the preferred indent style and width of the formatter.

## `rome.json`

#### Files

##### `files.maxSize`

The maximum allowed size for source code files in bytes. Files above
this limit will be ignored for performance reason. 

> Default: 1024

### Linter

#### `linter.enabled`

Enables Rome's linter

> Default: `true`

#### `linter.ignore`

An array of Unix shell style patterns.

```json
{
  "linter": {
    "ignore": ["scripts/*.js"]
  }
}
```

#### `linter.rules.recommended`

Enables the [recommended rules](/docs/lint/rules) for all categories.

> Default: `true`

#### `linter.rules.[category]`

Options that influence the rules of a single category. Rome supports the following categories:

{% include docs/reference/groups.md %}

#### `linter.rules.[category].recommended`

Enables the recommended rules for a single category.

Example:

```json
{
  "linter": {
    "enabled": true,
    "rules": {
      "nursery": {
        "recommended": true
      }
    }
  }
}
```

### Formatter

#### `formatter.enabled`

Enables Rome's formatter

> Default: `true`

#### `formatter.ignore`

An array of Unix shell style patterns.

```json
{
  "formatter": {
    "ignore": ["scripts/*.js"]
  }
}
```

#### `formatter.indentStyle`

The style of the indentation. It can be `"tab"` or `"space"`.

> Default: `tab`

Rome's default is `"tab"`.

#### `formatter.indentSize`

How big the indentation should be.

#### `formatter.lineWidth`

How many characters can be written on a single line.

> Default: `80`

### JavaScript

#### `javascript.formatter.quoteStyle`

The type of quote used when representing string literals. It can be `single` or `double`.

> Default: `double`

#### `javascript.formatter.quoteProperties`

When properties inside objects should be quoted. It can be `asNeeded` or `preserve`.

> Default: `asNeeded`

#### `javascript.formatter.trailingComma`

Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Possible values:
- `all`, the trailing comma is always added
- `es5`, the trailing comma is added only in places where it's supported by older version of JavaScript
- `none`, trailing commas are never added

> Default: `all`
