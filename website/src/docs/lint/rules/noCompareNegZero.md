---
title: Lint Rule noCompareNegZero
layout: layouts/rule.liquid
---

# noCompareNegZero (since v0.7.0)

> This rule is recommended by Rome.

Disallow comparing against `-0`

## Examples

### Invalid

```jsx
(1 >= -0)
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noCompareNegZero/">correctness/noCompareNegZero</a></span><span style="color: Tomato;">]</span><em>: </em><em>Do not use the &gt;= operator to compare against -0.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noCompareNegZero.js:1:2
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> (<span style="color: Tomato;">1</span><span style="color: Tomato;"> </span><span style="color: Tomato;">&gt;</span><span style="color: Tomato;">=</span><span style="color: Tomato;"> </span><span style="color: Tomato;">-</span><span style="color: Tomato;">0</span>)
  <span style="color: rgb(38, 148, 255);">│</span>  <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Safe fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Replace -0 with 0</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">(1 &gt;= -0)</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">(1 &gt;= 0)</span>

</code></pre>{% endraw %}

### Valid

```jsx
(1 >= 0)
```

