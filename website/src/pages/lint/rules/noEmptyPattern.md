---
title: Lint Rule noEmptyPattern
---

# noEmptyPattern (since v0.7.0)

> This rule is recommended by Rome.

Disallows empty destructuring patterns.

## Examples

### Invalid

```jsx
var {} = foo;
```

<pre class="language-text"><code class="language-text">correctness/noEmptyPattern.js:1:5 <a href="https://docs.rome.tools/lint/rules/noEmptyPattern">lint/correctness/noEmptyPattern</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected empty object pattern.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>var {} = foo;
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

```jsx
var {a: {}} = foo;
```

<pre class="language-text"><code class="language-text">correctness/noEmptyPattern.js:1:9 <a href="https://docs.rome.tools/lint/rules/noEmptyPattern">lint/correctness/noEmptyPattern</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected empty object pattern.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>var {a: {}} = foo;
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

```jsx
function foo({}) {}
```

<pre class="language-text"><code class="language-text">correctness/noEmptyPattern.js:1:14 <a href="https://docs.rome.tools/lint/rules/noEmptyPattern">lint/correctness/noEmptyPattern</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Unexpected empty object pattern.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>function foo({}) {}
   <strong>   │ </strong>             <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>

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

