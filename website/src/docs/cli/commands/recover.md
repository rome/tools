---
title: rome recover
layout: layouts/page.liquid
---

# `rome recover`

Whenever Rome needs to write files to the disk, for example when updating the formatting or autofixing a file, we first save a copy of the original file to an internal cache that we call the "recovery store". This is to allow you to revert your changes if necessary. This command is used to interact with this store.

We only keep the content of the last 5 commands that modified files. After that we will delete the oldest entry.

## Subcommands

### `rome recover list`

Show the contents of the recovery store. Including the command that was ran, at what time, files that were changed, and the `recover` commands you can use to perform operations.

**Example output**

<pre class="language-text"><code class="language-text"><span style="color: CornflowerBlue">$</span> rome recover list

<strong> Recovery stores </strong>

 <strong>1595570309210-lint-0</strong> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  <strong>Ran </strong><strong>42m21s</strong><strong> ago</strong> <span style="opacity: 0.8;">(2020-07-24T05:58:29.210Z)</span>"
  <span style="opacity: 0.8;">$ rome lint --apply</span>"

  <span style="opacity: 0.8;">- </span><span style="text-decoration-style: dotted;">src/App.ts</span>
  <span style="opacity: 0.8;">- </span><span style="text-decoration-style: dotted;">src/UserPage.ts</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">To select specific files to patch run:</span>
  <span style="opacity: 0.8;">$ rome recover apply 1595570309210-lint-0 --select</span>"

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">To see the changes with this patch run:</span>
  <span style="opacity: 0.8;">$ rome recover diff 1595570309210-lint-0</span>"

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">To apply </span><span style="color: DodgerBlue;"><strong>everything</strong></span><span style="color: DodgerBlue;"> in this patch run:</span>
  <span style="opacity: 0.8;">$ rome recover apply 1595570309210-lint-0</span>"

</pre></code>

### `rome recover pop`

Revert the last command. Equivalent to `rome recover apply \<MOST_RECENT_STORE_ID>`.

### `rome recover apply \<id>`

Revert the changes that were made by the corresponding `id`. You can find the `id` by running `rome recover list`.

Running this command will also produce a new store entry with the files that were reverted.

### `rome recover diff \<id>`

Produce a diff of changes between existing files and those included in the `id` store.

### `rome recover clear`

Clear the entire contents of the recovery store.
