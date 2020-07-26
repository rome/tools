---
title: rome config
layout: layouts/page.liquid
---

# `rome config`

This command works with all Rome project config locations (see [supported locations](/docs/project-config#supported-locations) for more info). When formatting a project config written with [RJSON](/docs/rjson), comments will be retained.

Before your project config is saved, we will validate it for errors. It is not possible to produce an invalid config with `rome config`.

## Subcommands

### `rome config enable <key>`

Set the `key` to `true`.

### `rome config disable <key>`

Set the `key` to `false`.

### `rome config set <key> <value>`

Set the `key` to a string `value`.

### `rome config set-directory <key> <value>`

Set the `key` to the string `value`. If `value` is an absolute path then it will be made relative to the project base directory.

### `rome config push <key> <value>`

Push the string `value` to an array at `key`. If `key` doesn't exist then it will be created.
