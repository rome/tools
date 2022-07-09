---
title: Lint Rule noLabelVar
layout: layouts/rule.liquid
---

# noLabelVar

Disallow labels that share a name with a variable

## Examples

### Invalid

```jsx
const x1 = "test";
x1: expr;
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;">noLabelVar</span><span style="color: Orange;">]</span><em>: </em><em>Do not use the </em><em><em>x1</em></em><em> variable name as a label</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> noLabelVar.js:2:1
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> const x1 = &quot;test&quot;;
  <span style="color: rgb(38, 148, 255);">│</span>       <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">The variable is declared here</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> x1: expr;
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

=  note: Creating a label with the same name as an in-scope variable leads to confusion.

</code></pre>{% endraw %}

### Valid

```jsx
const x = "test";
z: expr;
```

