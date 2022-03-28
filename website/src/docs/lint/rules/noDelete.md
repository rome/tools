---
title: Lint Rule noDelete
layout: layouts/rule.liquid
---

# noDelete (since v0.7.0)

Disallow the use of the `delete` operator

## Examples

### Invalid

```jsx
const arr = [['a','b','c'], [1, 2, 3]];
delete arr[0][2];
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noDelete/">js/noDelete</a></span><span style="color: Orange;">]</span><em>: </em><em>This is an unexpected use of the </em><em><em>delete</em></em><em> operator.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noDelete.js:2:1
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> delete arr[0][2];
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Replace with undefined assignment</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1,2 +1,2 @@</span>
0 0 |   const arr = [['a','b','c'], [1, 2, 3]];
1   | <span style="color: Tomato;">- </span><span style="color: Tomato;">delete arr[0][2];</span>
  1 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">arr[0][2] = undefined;</span>

</code></pre>{% endraw %}

```jsx
const obj = {a: {b: {c: 123}}};
delete obj.a.b.c;
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noDelete/">js/noDelete</a></span><span style="color: Orange;">]</span><em>: </em><em>This is an unexpected use of the </em><em><em>delete</em></em><em> operator.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noDelete.js:2:1
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> delete obj.a.b.c;
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Replace with undefined assignment</span>
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

