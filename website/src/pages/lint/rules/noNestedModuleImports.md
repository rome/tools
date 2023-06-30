---
title: Lint Rule noNestedModuleImports
parent: lint/rules/index
---

# noNestedModuleImports (since vnext)

Forbids importing from nested modules.

For larger code bases, it can be undesirable to let any file arbitrarily import any other
files. Arbitrary imports can lead to cycles that may be hard to debug, so it may be
advisable to specify which files may import from which other files.

A useful rule of thumb is that for modules that consist of several files, only the module's
`index.js` or `index.ts` may be imported directly from outside that module, while symbols
from other files should only be considered "public" if they're re-exported from the index.

This rule treats nested imports as an attempt to access "private" internals of a module.
Only exports defined by the `index.js` or `index.ts` are allowed to be imported externally.
Effectively, this means that you may not directly import any files or subdirectories that
are not siblings to the file you're in, or any of its ancestors.

This rule only applies to relative imports, since the API from external dependencies is
often out of your control.

## Examples

### Invalid

```jsx
import { privateInternals } from "../aunt/cousin";
```

<pre class="language-text"><code class="language-text">nursery/noNestedModuleImports.js:1:34 <a href="https://docs.rome.tools/lint/rules/noNestedModuleImports">lint/nursery/noNestedModuleImports</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Importing from nested modules is not allowed.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>import { privateInternals } from &quot;../aunt/cousin&quot;;
   <strong>   │ </strong>                                 <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Please import from </span><span style="color: rgb(38, 148, 255);"><strong>../aunt</strong></span><span style="color: rgb(38, 148, 255);"> instead (you may need to re-export from </span><span style="color: rgb(38, 148, 255);"><strong>../aunt/cousin</strong></span><span style="color: rgb(38, 148, 255);">).</span>
  
</code></pre>

### Valid

```jsx
import { publicExport } from "./sibling";
import { reexportedInternals } from "../aunt";
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
