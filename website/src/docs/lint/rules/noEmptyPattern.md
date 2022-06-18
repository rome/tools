---
title: Lint Rule noEmptyPattern
layout: layouts/rule.liquid
---

# noEmptyPattern

Disallows empty destructuring patterns.

## Examples

### Valid

```jsx
var {a = {}} = foo;
```

```jsx
var {a, b = {}} = foo;
```

```jsx
var {a = []} = foo;
```

```jsx
function foo({a = {}}) {}
```

```jsx
function foo({a = []}) {}
```

```jsx
var [a] = foo;
```

### Invalid

```jsx
var {} = foo;
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;">noEmptyPattern</span><span style="color: Orange;">]</span><em>: </em><em>Unexpected empty object pattern.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> noEmptyPattern.js:1:5
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> var {} = foo;
  <span style="color: rgb(38, 148, 255);">│</span>     <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

</code></pre>{% endraw %}

```jsx
var [] = foo;
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;">noEmptyPattern</span><span style="color: Orange;">]</span><em>: </em><em>Unexpected empty array pattern.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> noEmptyPattern.js:1:5
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> var [] = foo;
  <span style="color: rgb(38, 148, 255);">│</span>     <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

</code></pre>{% endraw %}

```jsx
var {a: {}} = foo;
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;">noEmptyPattern</span><span style="color: Orange;">]</span><em>: </em><em>Unexpected empty object pattern.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> noEmptyPattern.js:1:9
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> var {a: {}} = foo;
  <span style="color: rgb(38, 148, 255);">│</span>         <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

</code></pre>{% endraw %}

```jsx
var {a, b: {}} = foo;
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;">noEmptyPattern</span><span style="color: Orange;">]</span><em>: </em><em>Unexpected empty object pattern.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> noEmptyPattern.js:1:12
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> var {a, b: {}} = foo;
  <span style="color: rgb(38, 148, 255);">│</span>            <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

</code></pre>{% endraw %}

```jsx
var {a: []} = foo;
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;">noEmptyPattern</span><span style="color: Orange;">]</span><em>: </em><em>Unexpected empty array pattern.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> noEmptyPattern.js:1:9
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> var {a: []} = foo;
  <span style="color: rgb(38, 148, 255);">│</span>         <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

</code></pre>{% endraw %}

```jsx
function foo({}) {}
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;">noEmptyPattern</span><span style="color: Orange;">]</span><em>: </em><em>Unexpected empty object pattern.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> noEmptyPattern.js:1:14
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> function foo({}) {}
  <span style="color: rgb(38, 148, 255);">│</span>              <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

</code></pre>{% endraw %}

```jsx
function foo([]) {}
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;">noEmptyPattern</span><span style="color: Orange;">]</span><em>: </em><em>Unexpected empty array pattern.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> noEmptyPattern.js:1:14
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> function foo([]) {}
  <span style="color: rgb(38, 148, 255);">│</span>              <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

</code></pre>{% endraw %}

```jsx
function foo({a: {}}) {}
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;">noEmptyPattern</span><span style="color: Orange;">]</span><em>: </em><em>Unexpected empty object pattern.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> noEmptyPattern.js:1:18
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> function foo({a: {}}) {}
  <span style="color: rgb(38, 148, 255);">│</span>                  <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

</code></pre>{% endraw %}

```jsx
function foo({a: []}) {}
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;">noEmptyPattern</span><span style="color: Orange;">]</span><em>: </em><em>Unexpected empty array pattern.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> noEmptyPattern.js:1:18
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> function foo({a: []}) {}
  <span style="color: rgb(38, 148, 255);">│</span>                  <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

</code></pre>{% endraw %}

