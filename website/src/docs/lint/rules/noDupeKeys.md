---
title: Lint Rule noDupeKeys
layout: layouts/rule.liquid
---

# noDupeKeys (since v10.0.0)

Prevents object literals having more than one property declaration for the same key.
If the same key is specified multiple times, only the last property declaration will prevail and previous will be lost, which is likely a mistake.

## Examples

### Invalid

```jsx
const obj = {
   	a: 1,
   	a: 2,
}
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noDupeKeys.js:2:5 <a href="https://rome.tools/docs/lint/rules/noDupeKeys">lint/nursery/noDupeKeys</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This property value is later overwritten by a property with the same name.</span>
  
    <strong>1 │ </strong>const obj = {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>   	a: 1,
   <strong>   │ </strong>   	<strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>   	a: 2,
    <strong>4 │ </strong>}
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Overwritten with this value.</span>
  
    <strong>1 │ </strong>const obj = {
    <strong>2 │ </strong>   	a: 1,
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>   	a: 2,
   <strong>   │ </strong>   	<strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>}
    <strong>5 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">If an object property or with the same key is defined multiple times (except when combining a getter with a setter), only the last definition makes it into the object and previous definitions are ignored.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove this property value</span>
  
  
</code></pre>{% endraw %}

```jsx
const obj = {
   	set a(v) {},
   	a: 2,
}
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noDupeKeys.js:2:5 <a href="https://rome.tools/docs/lint/rules/noDupeKeys">lint/nursery/noDupeKeys</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">This property setter is later overwritten by a property with the same name.</span>
  
    <strong>1 │ </strong>const obj = {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>   	set a(v) {},
   <strong>   │ </strong>   	<strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>   	a: 2,
    <strong>4 │ </strong>}
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Overwritten with this value.</span>
  
    <strong>1 │ </strong>const obj = {
    <strong>2 │ </strong>   	set a(v) {},
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>   	a: 2,
   <strong>   │ </strong>   	<strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>}
    <strong>5 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">If an object property or with the same key is defined multiple times (except when combining a getter with a setter), only the last definition makes it into the object and previous definitions are ignored.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove this property setter</span>
  
  
</code></pre>{% endraw %}

### Valid

```jsx
const obj = {
   	a: 1,
   	b: 2,
}
```

```jsx
const obj = {
   	get a() { return 1; },
   	set a(v) {},
}
```

