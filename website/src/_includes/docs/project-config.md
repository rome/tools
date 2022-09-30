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

#### `linter.ignore`

An array of Unix shell style patterns.

```json
{
  "linter": {
    "ignore": ["scripts/*.js"]
  }
}
```


> Default: `true`

#### `linter.rules.recommended`

Enables the [recommended rules](/docs/lint/rules) for all the groups. 

> Default: `true`


#### `linter.rules.correctness` 

A list of rules for `Correctness` category.  

#### `linter.rules.correctness.recommended` 

Enables the [recommended rules](/docs/lint/rules) for the category `Correctness`.

Example:

```json
{
  "linter": {
    "enabled": true,
    "rules": {
      "correctness": {
        "noDebugger": "off"
      }
    }
  }
}

```


#### `linter.rules.style`

A list of rules for `Style` category.

#### `linter.rules.style.recommended`

Enables the [recommended rules](/docs/lint/rules) for the category `Style`.


Example:

```json
{
  "linter": {
    "enabled": true,
    "rules": {
      "style": {
        "noNegationElse": "off"
      }
    }
  }
}

```


#### `linter.rules.nursery`

A list of rules for `Nursery` category.

#### `linter.rules.nursery.recommended`

Enables the [recommended rules](/docs/lint/rules) for the category `Nursery`.

Example:

```json
{
  "linter": {
    "enabled": true,
    "rules": {
      "nursery": {
        "noCommentText": "on"
      }
    }
  }
}

```

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

#### `javascript.formatter.quoteStyle`

The type of quote used when representing string literals. It can be `single` or `double`.

> Default: `double`

#### `javascript.formatter.quoteProperties`

When properties inside objects should be quoted. It can be `asNeeded` or `preserve`.

> Default: `asNeeded`


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
        "noDeadCode": "error",
        "useCamelCase": "warn"
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
        "noCommentText": "off"
      },
      "regex": {
        "noMultipleSpacesInRegularExpressionLiterals": "off"
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
        "noCommentText": "warn"
      },
      "regex": {
        "noMultipleSpacesInRegularExpressionLiterals": "error"
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
        "noCommentText": {
          "level": "warn",
          "options": {}
        }
      }
    }
  }
}
```

- `level` will indicate the severity of the diagnostic, valid values are: `"off"`, `"warn"` and `"error"`;
- `options` is a wildcard value, meaning that will change based on the rule;
