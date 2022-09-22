---
title: Lint Rule noLabelVar
layout: layouts/rule.liquid
---

# noLabelVar (since v0.7.0)

> This rule is recommended by Rome.

Disallow labels that share a name with a variable

## Examples

### Invalid

```jsx
const x1 = "test";
x1: expr;
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noLabelVar.js:2:1 <a href="https://rome.tools/docs/lint/rules/noLabelVar">lint/correctness/noLabelVar</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Do not use the </span><span style="color: Tomato;"><strong>x1</strong></span><span style="color: Tomato;"> variable name as a label</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noLabelVar.js:2:1
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">x</span><span style="color: Tomato;">1</span>: expr;
    <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The variable is declared here</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noLabelVar.js:1:7
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> const <span style="color: Tomato;">x</span><span style="color: Tomato;">1</span> = &quot;test&quot;;
    <span style="color: rgb(38, 148, 255);">│</span>       <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Creating a label with the same name as an in-scope variable leads to confusion.</span>
  
</code></pre>{% endraw %}

### Valid

```jsx
const x = "test";
z: expr;
```

