## Project Configuration

The configuration file is considered **optional**, Rome has good defaults. Use the configuration
file to change those defaults.

The configuration file must be placed at the root of your project, usually at the same directory level of your
`package.json`. The name of the file must be `rome.json`.

All properties are **optional**, you can even have an empty config!

We are deliberately lean with the supported configuration. 
We do not include options just for the sake of personalization. 
We aim to offer everything out of the box and only introduce configuration if [absolutely necessary](https://rome.tools/#philosophy).

```json
{
  "formatter": {
    "indentStyle": "tab",
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

#### `linter.rules.recommended`

Enables the [recommended rules](/docs/lint/rules) for all the groups. 

> Default: `true`


#### `linter.rules.js` 

A list of rules for `JavaScript` category.  

#### `linter.rules.js.recommended` 

Enables the [recommended rules](/docs/lint/rules) for the category `JavaScript`.

#### `linter.rules.js.rules`

A map with the name of the rule as key, and their configuration as value. Check [rule configuration section](#configure-a-rule).
for more details.

Example:

```json
{
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


#### `linter.rules.ts`

A list of rules for `TypeScript` category.

#### `linter.rules.ts.recommended`

Enables the [recommended rules](/docs/lint/rules) for the category `TypeScript`.

#### `linter.rules.ts.rules`

A map with the name of the rule as key, and their configuration as value. Check [rule configuration section](#configure-a-rule).
for more details.

Example:

```json
{
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


#### `linter.rules.jsx`

A list of rules for `JSX` category.

#### `linter.rules.jsx.recommended`

Enables the [recommended rules](/docs/lint/rules) for the category `JSX`.

#### `linter.rules.jsx.rules`

A map with the name of the rule as key, and their configuration as value. Check [rule configuration section](#configure-a-rule).
for more details.

Example:

```json
{
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


#### `linter.rules.regex`

A list of rules for `Regex` category.

#### `linter.rules.regex.recommended`

Enables the [recommended rules](/docs/lint/rules) for the category `Regex`.

#### `linter.rules.regex.rules`

A map with the name of the rule as key, and their configuration as value. Check [rule configuration section](#configure-a-rule).
for more details.

Example:

```json
{
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

#### `formatter.enabled`

Enables Rome's formatter

> Default: `true`


#### `formatter.indentStyle`

The style of the indentation. It can be `"tab"` or `"space"`.

> Default: `tab`

Rome's default is `"tab"`.

#### `formatter.indentSize`

How big the indentation should be.

#### `formatter.lineWidth`

How many characters can be written on a single line.

> Default: `80`

#### `javascript.formatter.quoteStyle`

The type of quote used when representing string literals. It can be `single` or `double`.

> Default: `double`

### Configure a rule

A rule can be configured for multiple purposes:
- change the severity of their diagnostics;
- turn the rule off;
- pass possible options to customize the rule;

#### Turn a rule on

Rules that are recommended are enabled by default. Rules that are not recommended
are not enabled, but they should be enabled via configuration.

To enable rules, you need to change their diagnostics severity based on your needs:

```json
{
  "linter": {
    "enabled": true,
    "rules": {
      "js": {
        "rules": {
          "noDeadCode": "error",
          "useCamelCase": "warn"
        }
      }
    }
  }
}
```

#### Turn a rule off

Just add `"off"` as value inside its configuration. For example:

```json
{
  "linter": {
    "enabled": true,
    "rules": {
      "jsx": {
        "rules": {
          "noCommentText": "off"
        }
      },
      "regex": {
        "rules": {
          "noMultipleSpacesInRegularExpressionLiterals": "off"
        }
      }
    }
  }
}
```

#### Change severity of diagnostics

Most of Rome's rules will emit an **error**, but you are free to change their severity.
Just add `"warn"` as value of the rule. Example:

```json
{
  "linter": {
    "enabled": true,
    "rules": {
      "jsx": {
        "rules": {
          "noCommentText": "warn"
        }
      },
      "regex": {
        "rules": {
          "noMultipleSpacesInRegularExpressionLiterals": "error"
        }
      }
    }
  }
}
```

This is useful in cases there's being a refactor going on and there's need to make the 
CI passing.

#### Pass options to a rule

Not all the rules require options, but when they do *accept* some, you can pass them
by shaping the value of the rule in a different way.

```json
{
  "linter": {
    "enabled": true,
    "rules": {
      "jsx": {
        "rules": {
          "noCommentText": {
            "level": "warn",
            "options": {}
          }
        }
      }
    }
  }
}
```

- `level` will indicate the severity of the diagnostic, valid values are: `"off"`, `"warn"` and `"error"`;
- `options` is a wildcard value, meaning that will change based on the rule;
