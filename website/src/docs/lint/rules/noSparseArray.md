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

{% raw %}<pre class="language-text"><code class="language-text">correctness/noSparseArray.js:1:1 <a href="https://rome.tools/docs/lint/rules/noSparseArray">lint/correctness/noSparseArray</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This </span><span style="color: Tomato;"><strong>array</strong></span><span style="color: Tomato;"> contains an </span><span style="color: Tomato;"><strong>empty slot</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>[1,,2]
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Replace hole with undefined</span>
  
      | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
  0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">[1,,2]</span>
    0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">[1, undefined,2]</span>
  
</code></pre>{% endraw %}

