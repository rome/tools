---
title: Lint Rule noUnusedTemplateLiteral
layout: layouts/rule.liquid
---

# noUnusedTemplateLiteral (since v0.7.0)

> This rule is recommended by Rome.

Disallow template literals if interpolation and special-character handling are not needed

## Examples

### Invalid

```jsx
const foo = `bar`
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noUnusedTemplateLiteral/">js/noUnusedTemplateLiteral</a></span><span style="color: Tomato;">]</span><em>: </em><em>Do not use template literals if interpolation and special-character handling are not needed.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noUnusedTemplateLiteral.js:1:13
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> const foo = <span style="color: Tomato;">`</span><span style="color: Tomato;">b</span><span style="color: Tomato;">a</span><span style="color: Tomato;">r</span><span style="color: Tomato;">`</span>
  <span style="color: rgb(38, 148, 255);">│</span>             <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Replace with string literal</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">const foo = `bar`</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">const foo = &quot;bar&quot;</span>

</code></pre>{% endraw %}

```jsx
const foo = `bar `
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noUnusedTemplateLiteral/">js/noUnusedTemplateLiteral</a></span><span style="color: Tomato;">]</span><em>: </em><em>Do not use template literals if interpolation and special-character handling are not needed.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noUnusedTemplateLiteral.js:1:13
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> const foo = <span style="color: Tomato;">`</span><span style="color: Tomato;">b</span><span style="color: Tomato;">a</span><span style="color: Tomato;">r</span><span style="color: Tomato;"> </span><span style="color: Tomato;">`</span>
  <span style="color: rgb(38, 148, 255);">│</span>             <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Replace with string literal</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">const foo = `bar `</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">const foo = &quot;bar &quot;</span>

</code></pre>{% endraw %}

### Valid

```jsx
const foo = `bar
has newline`;
```

```jsx
const foo = `"bar"`
```

```jsx
const foo = `'bar'`
```

