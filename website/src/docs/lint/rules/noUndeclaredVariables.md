---
title: Lint Rule noUndeclaredVariables
layout: layouts/rule.liquid
---

# noUndeclaredVariables (since v0.10.0)

Prevents the usage of variables that haven't been declared inside the document

## Examples

### Invalid

```jsx
foobar;
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noUndeclaredVariables.js:1:1 <a href="https://rome.tools/docs/lint/rules/noUndeclaredVariables">lint/nursery/noUndeclaredVariables</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">The </span><span style="color: Orange;"><strong>foobar</strong></span><span style="color: Orange;"> variable is undeclared</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noUndeclaredVariables.js:1:1
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span><span style="color: Tomato;">b</span><span style="color: Tomato;">a</span><span style="color: Tomato;">r</span>;
    <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
</code></pre>{% endraw %}

