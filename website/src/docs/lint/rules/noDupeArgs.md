---
title: Lint Rule noDupeArgs
layout: layouts/rule.liquid
---

# noDupeArgs (since v0.8.0)

> This rule is recommended by Rome.

Disallow duplicate function arguments name.

## Examples

### Invalid

```jsx
var f = function(a, b, b) {}
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noDupeArgs/">js/noDupeArgs</a></span><span style="color: Orange;">]</span><em>: </em><em>Duplicate argument name</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noDupeArgs.js:1:24
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> var f = function(a, b, b) {}
  <span style="color: rgb(38, 148, 255);">│</span>                        <span style="color: rgb(38, 148, 255);">-</span>

</code></pre>{% endraw %}

```jsx
function b(a, b, b) {}
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noDupeArgs/">js/noDupeArgs</a></span><span style="color: Orange;">]</span><em>: </em><em>Duplicate argument name</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noDupeArgs.js:1:18
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> function b(a, b, b) {}
  <span style="color: rgb(38, 148, 255);">│</span>                  <span style="color: rgb(38, 148, 255);">-</span>

</code></pre>{% endraw %}

### Valid

```jsx
function i(i, b, c) {}
var j = function (j, b, c) {};
function k({ k, b }, { c, d }) {}
function l([, l]) {}
function foo([[a, b], [c, d]]) {}
```

