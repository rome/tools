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

{% raw %}<pre class="language-text"><code class="language-text">correctness/noCatchAssign.js:5:3 <a href="https://rome.tools/docs/lint/rules/noCatchAssign">lint/correctness/noCatchAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;"> Do not </span><span style="color: Tomato;"><strong>reassign catch parameters.</strong></span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noCatchAssign.js:5:3
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">5</span> <span style="color: rgb(38, 148, 255);">│</span>   <span style="color: Tomato;">e</span> = 10;
    <span style="color: rgb(38, 148, 255);">│</span>   <span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The catch parameter is declared here</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noCatchAssign.js:3:10
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">3</span> <span style="color: rgb(38, 148, 255);">│</span> } catch (<span style="color: Tomato;">e</span>) {
    <span style="color: rgb(38, 148, 255);">│</span>          <span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Use a local variable instead.</span>
  
</code></pre>{% endraw %}

### Valid

```jsx
try {

} catch (e) {
  let e = 10;
  e = 100;
}
```

