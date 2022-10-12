---
title: Lint Rule noConstAssign
layout: layouts/rule.liquid
---

# noConstAssign (since v10.0.0)

Prevents from having `const` variables being re-assigned.

Trying to assign a value to a `const` will cause an `TypeError` at runtime.

## Examples

### Invalid

```jsx
const a = 1;
a = 4;
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noConstAssign.js:2:1 <a href="https://rome.tools/docs/lint/rules/noConstAssign">lint/nursery/noConstAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Can't assign </span><span style="color: Orange;"><strong>a</strong></span><span style="color: Orange;"> because it's a constant</span>
  
    <strong>1 │ </strong>const a = 1;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>a = 4;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">This is where the variables is defined as constant</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>const a = 1;
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>a = 4;
    <strong>3 │ </strong>
  
</code></pre>{% endraw %}

```jsx
const a = 2;
a += 1;
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noConstAssign.js:2:1 <a href="https://rome.tools/docs/lint/rules/noConstAssign">lint/nursery/noConstAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Can't assign </span><span style="color: Orange;"><strong>a</strong></span><span style="color: Orange;"> because it's a constant</span>
  
    <strong>1 │ </strong>const a = 2;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>a += 1;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">This is where the variables is defined as constant</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>const a = 2;
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>a += 1;
    <strong>3 │ </strong>
  
</code></pre>{% endraw %}

```jsx
const a = 1;
++a;
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noConstAssign.js:2:3 <a href="https://rome.tools/docs/lint/rules/noConstAssign">lint/nursery/noConstAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Can't assign </span><span style="color: Orange;"><strong>a</strong></span><span style="color: Orange;"> because it's a constant</span>
  
    <strong>1 │ </strong>const a = 1;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>++a;
   <strong>   │ </strong>  <strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">This is where the variables is defined as constant</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>const a = 1;
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>++a;
    <strong>3 │ </strong>
  
</code></pre>{% endraw %}

