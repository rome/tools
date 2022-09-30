---
title: Lint Rule noDelete
layout: layouts/rule.liquid
---

# noDelete (since v0.7.0)

> This rule is recommended by Rome.

Disallow the use of the `delete` operator

## Examples

### Invalid

```jsx
const arr = [['a','b','c'], [1, 2, 3]];
delete arr[0][2];
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noDelete.js:2:1 <a href="https://rome.tools/docs/lint/rules/noDelete">lint/correctness/noDelete</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This is an unexpected use of the </span><span style="color: Tomato;"><strong>delete</strong></span><span style="color: Tomato;"> operator.</span>
  
    <strong>1 │ </strong>const arr = [['a','b','c'], [1, 2, 3]];
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>delete arr[0][2];
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Replace with undefined assignment</span>
  
      | <span style="color: rgb(38, 148, 255);">@@ -1,2 +1,2 @@</span>
  0 0 |   const arr = [['a','b','c'], [1, 2, 3]];
  1   | <span style="color: Tomato;">- </span><span style="color: Tomato;">delete arr[0][2];</span>
    1 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">arr[0][2] = undefined;</span>
  
</code></pre>{% endraw %}

```jsx
const obj = {a: {b: {c: 123}}};
delete obj.a.b.c;
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noDelete.js:2:1 <a href="https://rome.tools/docs/lint/rules/noDelete">lint/correctness/noDelete</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">This is an unexpected use of the </span><span style="color: Tomato;"><strong>delete</strong></span><span style="color: Tomato;"> operator.</span>
  
    <strong>1 │ </strong>const obj = {a: {b: {c: 123}}};
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>delete obj.a.b.c;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Replace with undefined assignment</span>
  
      | <span style="color: rgb(38, 148, 255);">@@ -1,2 +1,2 @@</span>
  0 0 |   const obj = {a: {b: {c: 123}}};
  1   | <span style="color: Tomato;">- </span><span style="color: Tomato;">delete obj.a.b.c;</span>
    1 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">obj.a.b.c = undefined;</span>
  
</code></pre>{% endraw %}

### Valid

```jsx
const foo = new Set([1,2,3]);
foo.delete(1);
```

