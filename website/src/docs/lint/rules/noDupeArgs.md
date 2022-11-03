---
title: Lint Rule noDupeArgs
layout: layouts/page.liquid
---

# noDupeArgs (since v0.9.0)

> This rule is recommended by Rome.

Disallow duplicate function arguments name.

## Examples

### Invalid

```jsx
var f = function(a, b, b) {}
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noDupeArgs.js:1:24 <a href="https://rome.tools/docs/lint/rules/noDupeArgs">lint/correctness/noDupeArgs</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Duplicate argument name</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>var f = function(a, b, b) {}
   <strong>   │ </strong>                       <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>{% endraw %}

```jsx
function b(a, b, b) {}
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noDupeArgs.js:1:18 <a href="https://rome.tools/docs/lint/rules/noDupeArgs">lint/correctness/noDupeArgs</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Duplicate argument name</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function b(a, b, b) {}
   <strong>   │ </strong>                 <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>{% endraw %}

### Valid

```jsx
function i(i, b, c) {}
var j = function (j, b, c) {};
function k({ k, b }, { c, d }) {}
function l([, l]) {}
function foo([[a, b], [c, d]]) {}
```

