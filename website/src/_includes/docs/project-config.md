## Project Configuration

The configuration file is considered **optional**, Rome has strong defaults. Use the configuration
file to change those defaults.

The configuration file must be placed at the root of your project, usually at the same directory level of your
`package.json`. The name of the file must be `rome.json`.

All properties are **optional**, you can even have an empty config!

We are deliberately lean with the supported configuration. We do not include options just for the sake of personalization. We aim to offer everything out of the box and only introduce configuration if absolutely necessary.

```json
{
  "root": true,
  "formatter": {
    "identStyle": "tab",
    "lineWidth": 120
  },
  "linter": {
    "enabled": false
  }
}
```

### Properties

#### `linter.enabled`

Enables Rome's linter

> Default: `true`

#### `lint.rules.recommended`

Enables the recommended rule sets for all the groups. 

> Default: `true`


#### `lint.rules.js` 

A list of rules for `JavaScript` category.  

#### `lint.rules.js.recommended` 

Enables the recommended rules for the category `JavaScript`.

#### `lint.rules.js.rules`

A map with the name of the rule as key, and their configuration as value. Check [#configure-a-rule]
for more details.

Example:

```json
{
  "root": true,
  "linter": {
    "enabled": true,
    "rules": {
      "js": {
        "rules": {
          "noDebugger": "off"
        }
      }
    }
  }
}

```


#### `lint.rules.ts`

A list of rules for `TypeScript` category.

#### `lint.rules.ts.recommended`

Enables the recommended rules for the category `TypeScript`.

#### `lint.rules.ts.rules`

A map with the name of the rule as key, and their configuration as value. Check [#configure-a-rule]
for more details.

Example:

```json
{
  "root": true,
  "linter": {
    "enabled": true,
    "rules": {
      "ts": {
        "rules": {
          "useShorthandArrayType": "off"
        }
      }
    }
  }
}

```


#### `lint.rules.jsx`

A list of rules for `JSX` category.

#### `lint.rules.jsx.recommended`

Enables the recommended rules for the category `JSX`.

#### `lint.rules.jsx.rules`

A map with the name of the rule as key, and their configuration as value. Check [#configure-a-rule]
for more details.

Example:

```json
{
  "root": true,
  "linter": {
    "enabled": true,
    "rules": {
      "jsx": {
        "rules": {
          "noCommentText": "off"
        }
      }
    }
  }
}

```


#### `lint.rules.regex`

A list of rules for `Regex` category.

#### `lint.rules.regex.recommended`

Enables the recommended rules for the category `Regex`.

#### `lint.rules.regex.rules`

A map with the name of the rule as key, and their configuration as value. Check [#configure-a-rule]
for more details.

Example:

```json
{
  "root": true,
  "linter": {
    "enabled": true,
    "rules": {
      "regex": {
        "rules": {
          "noMultipleSpacesInRegularExpressionLiterals": "off"
        }
      }
    }
  }
}

```

### `formatter.enabled`

Enables Rome's formatter

> Default: `true`


### `format.indentStyle`

The style of the indentation. It can be `"tab"` or `"space"`.

> Default: `tab`

Rome's default is `"tab"`.

### `format.indentSize`

How big the indentation should be.

### `format.lineWidth`

How many characters can be written on a single line.

> Default: `80`

### `javascript.formatter.quoteStyle`

The type of quote used when representing string literals. It can be `single` or `double`.

> Default: `double`

## Rule configuration