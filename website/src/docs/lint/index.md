---
title: Linting
layout: layouts/base.njk
---

# Linting

## Project Config

### `lint.ignore`

```text
lint: {
	ignore: []
}
```

This can also be configured via the [`rome config` command](/docs/cli/commands/config):

```bash
rome config push lint.ignore "some-path"
```

### `lint.globals`

```text
lint: {
	globals: []
}
```

This can also be configured via the [`rome config` command](/docs/cli/commands/config):

```bash
rome config push lint.globals SomeGlobal
```

## Formatting 

## Fixing

### Recommended Fixes

### Reviewing
