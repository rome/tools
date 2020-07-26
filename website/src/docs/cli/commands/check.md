---
title: rome check
layout: layouts/page.liquid
---

# `rome check`

## Flags

### `--apply`

### `--changed \<branch/commit>`

Only include files that were changed between the specified `branch/commit`. This can be useful for performance in large projects.

If the `branch/commit` is ommitted then we default to the default branch, either `main` or `master`. ie. `rome check --changed` is equivalent to `rome check --changed main`.
