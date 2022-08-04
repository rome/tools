---
title: Lint Rule noEmptyPattern
layout: layouts/rule.liquid
---

# noEmptyPattern (since v0.7.0)

> This rule is recommended by Rome.

Disallows empty destructuring patterns.

## Examples

### Invalid

```jsx
var {} = foo;
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noEmptyPattern/">js/noEmptyPattern</a></span><span style="color: Tomato;">]</span><em>: </em><em>Unexpected empty object pattern.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noEmptyPattern.js:1:5
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> var <span style="color: Tomato;">{</span><span style="color: Tomato;">}</span> = foo;
  <span style="color: rgb(38, 148, 255);">│</span>     <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

</code></pre>{% endraw %}

```jsx
var {a: {}} = foo;
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noEmptyPattern/">js/noEmptyPattern</a></span><span style="color: Tomato;">]</span><em>: </em><em>Unexpected empty object pattern.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noEmptyPattern.js:1:9
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> var {a: <span style="color: Tomato;">{</span><span style="color: Tomato;">}</span>} = foo;
  <span style="color: rgb(38, 148, 255);">│</span>         <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

</code></pre>{% endraw %}

```jsx
function foo({}) {}
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noEmptyPattern/">js/noEmptyPattern</a></span><span style="color: Tomato;">]</span><em>: </em><em>Unexpected empty object pattern.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noEmptyPattern.js:1:14
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> function foo(<span style="color: Tomato;">{</span><span style="color: Tomato;">}</span>) {}
  <span style="color: rgb(38, 148, 255);">│</span>              <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

</code></pre>{% endraw %}

### Valid

The following cases are valid because they create new bindings.

```jsx
var {a = {}} = foo;
var {a, b = {}} = foo;
var {a = []} = foo;
function foo({a = {}}) {}
function foo({a = []}) {}
var [a] = foo;
```

