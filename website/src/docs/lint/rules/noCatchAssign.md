---
title: Lint Rule noCatchAssign
layout: layouts/rule.liquid
---

# noCatchAssign (since v0.7.0)

> This rule is recommended by Rome.

Disallow reassigning exceptions in catch clauses

## Examples

### Invalid

```jsx
try {

} catch (e) {
  e;
  e = 10;
}
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noCatchAssign/">correctness/noCatchAssign</a></span><span style="color: Tomato;">]</span><strong>: </strong><strong> Do not </strong><strong><strong>reassign catch parameters.</strong></strong><strong></strong>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noCatchAssign.js:5:3
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">3</span> <span style="color: rgb(38, 148, 255);">│</span> } catch (e) {
  <span style="color: rgb(38, 148, 255);">│</span>          <span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">The catch parameter is declared here</span>
<span style="color: rgb(38, 148, 255);">4</span> <span style="color: rgb(38, 148, 255);">│</span>   e;
<span style="color: rgb(38, 148, 255);">5</span> <span style="color: rgb(38, 148, 255);">│</span>   <span style="color: Tomato;">e</span> = 10;
  <span style="color: rgb(38, 148, 255);">│</span>   <span style="color: Tomato;">^</span>

=  note: Use a local variable instead.

</code></pre>{% endraw %}

### Valid

```jsx
try {

} catch (e) {
  let e = 10;
  e = 100;
}
```

