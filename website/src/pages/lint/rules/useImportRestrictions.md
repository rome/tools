---
title: Lint Rule useImportRestrictions
parent: lint/rules/index
---

# useImportRestrictions (since vnext)

Disallows imports from certain modules.

This rules enforces the following restrictions:

## Package private visibility

All exported symbols are considered to be "package private". This means that modules that
reside in the same directory, as well as submodules of those "sibling" modules, are
allowed to import them, while any other modules that are further away in the file system
are restricted from importing them. A symbol's visibility may be extended by
re-exporting from an index file.

Notes:

- This rule only applies to relative imports, since the API from external dependencies is
often out of your control.
- This rule only applies to source imports. Imports for resources such as images or CSS
files are exempted.
- A future improvement will relax the restriction from "all exported symbols" to those
that have an `@package` JSDoc annotation.

This rule is intended to be extended with additional import restrictions.
Please see the tracking issue to follow progress: https://github.com/rome/tools/issues/4678

Source:

- https://github.com/uhyo/eslint-plugin-import-access

## Examples

### Invalid

```jsx
// Attempt to import from `foo.js` from outside its `sub` module.
import { fooPackageVariable } from "./sub/foo.js";

// Attempt to import from `bar.ts` from outside its `aunt` module.
import { barPackageVariable } from "../aunt/bar.ts";

// Assumed to resolve to a JS/TS file.
import { fooPackageVariable } from "./sub/foo";

// If the `sub/foo` module is inaccessible, so is its index file.
import { fooPackageVariable } from "./sub/foo/index.js";
```

<pre class="language-text"><code class="language-text">nursery/useImportRestrictions.js:2:36 <a href="https://docs.rome.tools/lint/rules/useImportRestrictions">lint/nursery/useImportRestrictions</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Importing package private symbols is not allowed from outside the module directory.</span>
  
    <strong>1 │ </strong>// Attempt to import from `foo.js` from outside its `sub` module.
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>import { fooPackageVariable } from &quot;./sub/foo.js&quot;;
   <strong>   │ </strong>                                   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
    <strong>4 │ </strong>// Attempt to import from `bar.ts` from outside its `aunt` module.
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Please import from </span><span style="color: rgb(38, 148, 255);"><strong>./sub</strong></span><span style="color: rgb(38, 148, 255);"> instead (you may need to re-export the symbol(s) from </span><span style="color: rgb(38, 148, 255);"><strong>./sub/foo.js</strong></span><span style="color: rgb(38, 148, 255);">).</span>
  
nursery/useImportRestrictions.js:5:36 <a href="https://docs.rome.tools/lint/rules/useImportRestrictions">lint/nursery/useImportRestrictions</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Importing package private symbols is not allowed from outside the module directory.</span>
  
    <strong>4 │ </strong>// Attempt to import from `bar.ts` from outside its `aunt` module.
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>import { barPackageVariable } from &quot;../aunt/bar.ts&quot;;
   <strong>   │ </strong>                                   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>6 │ </strong>
    <strong>7 │ </strong>// Assumed to resolve to a JS/TS file.
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Please import from </span><span style="color: rgb(38, 148, 255);"><strong>../aunt</strong></span><span style="color: rgb(38, 148, 255);"> instead (you may need to re-export the symbol(s) from </span><span style="color: rgb(38, 148, 255);"><strong>../aunt/bar.ts</strong></span><span style="color: rgb(38, 148, 255);">).</span>
  
</code></pre>

### Valid

```jsx
// Imports within the same module are always allowed.
import { fooPackageVariable } from "./foo.js";

// Imports within the same module are always allowed.
import { fooPackageVariable } from "./foo.js";

// Resources (anything other than JS/TS files) are exempt.
import { barResource } from "../aunt/bar.png";

// A parent index file is accessible like other modules.
import { internal } from "../../index.js";

// If the `sub` module is accessible, so is its index file.
import { subPackageVariable } from "./sub/index.js";

// Library imports are exempt.
import useAsync from "react-use/lib/useAsync";
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
