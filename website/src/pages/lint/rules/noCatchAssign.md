---
title: Lint Rule noCatchAssign
parent: lint/rules/index
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

<pre class="language-text"><code class="language-text">suspicious/noCatchAssign.js:5:3 <a href="https://docs.rome.tools/lint/rules/noCatchAssign">lint/suspicious/noCatchAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;"> Do not </span><span style="color: Tomato;"><strong>reassign catch parameters.</strong></span>
  
    <strong>3 │ </strong>} catch (e) {
    <strong>4 │ </strong>  e;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>  e = 10;
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong>
    <strong>6 │ </strong>}
    <strong>7 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The catch parameter is declared here</span>
  
    <strong>1 │ </strong>try {
    <strong>2 │ </strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>} catch (e) {
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>  e;
    <strong>5 │ </strong>  e = 10;
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Use a local variable instead.</span>
  
</code></pre>

### Valid

```jsx
try {

} catch (e) {
  let e = 10;
  e = 100;
}
```

