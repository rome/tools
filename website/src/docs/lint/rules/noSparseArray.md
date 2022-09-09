---
title: Lint Rule noSparseArray
layout: layouts/rule.liquid
---

# noSparseArray (since v0.7.0)

> This rule is recommended by Rome.

Disallow sparse arrays

## Examples

### Invalid

```jsx
[1,,2]
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noSparseArray/">correctness/noSparseArray</a></span><span style="color: Tomato;">]</span><em>: </em><em>This </em><em><em>array</em></em><em> contains an </em><em><em>empty slot</em></em><em>.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noSparseArray.js:1:1
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">[</span><span style="color: Tomato;">1</span><span style="color: Tomato;">,</span><span style="color: Tomato;">,</span><span style="color: Tomato;">2</span><span style="color: Tomato;">]</span>
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Replace hole with undefined</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">[1,,2]</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">[1, undefined,2]</span>

</code></pre>{% endraw %}

