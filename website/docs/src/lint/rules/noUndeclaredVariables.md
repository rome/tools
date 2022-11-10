---
title: Lint Rule noUndeclaredVariables
---

# noUndeclaredVariables (since v0.10.0)

Prevents the usage of variables that haven't been declared inside the document

## Examples

### Invalid

```jsx
foobar;
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noUndeclaredVariables.js:1:1 <a href="https://docs.rome.tools/lint/rules/noUndeclaredVariables">lint/correctness/noUndeclaredVariables</a> ━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">The </span><span style="color: Orange;"><strong>foobar</strong></span><span style="color: Orange;"> variable is undeclared</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>foobar;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
</code></pre>{% endraw %}

