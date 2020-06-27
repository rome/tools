---
title: CLI
layout: layouts/base.njk
---

# CLI

## Commands

- [`config`](/docs/cli/commands/config)
- [`init`](/docs/cli/commands/init)
- [`lint`](/docs/cli/commands/lint)
- [`logs`](/docs/cli/commands/logs)
- [`lsp`](/docs/cli/commands/lsp)
- [`noop`](/docs/cli/commands/noop)
- [`rage`](/docs/cli/commands/rage)
- [`start`](/docs/cli/commands/start)
- [`status`](/docs/cli/commands/status)
- [`stop`](/docs/cli/commands/stop)

## Global Flags

These are flags that can be added to any Rome command. 

### `--cwd <dir>`

### `--fieri`

Adds some flavor to output diagnostics.

### `--max-diagnostics <num>`

### `--show-all-diagnostics`

### `--silent`

### `--temporary-daemon`

### `--verbose-diagnostics`

## Debugging Flags

Debugging development flags are also available and documented in [debugging](/docs/cli/debugging).

## Shell Autocomplete

```bash
rome --generate-autocomplete bash
```
