---
title: Lint Rule noImplicitBoolean
layout: layouts/docs.liquid
---

# noImplicitBoolean (since v0.7.0)

> This rule is recommended by Rome.

Disallow implicit `true` values on JSX boolean attributes

## Examples

### Invalid

```jsx
<input disabled />
```

{% raw %}<pre class="language-text"><code class="language-text">style/noImplicitBoolean.js:1:8 <a href="https://docs.rome.tools/lint/rules/noImplicitBoolean">lint/style/noImplicitBoolean</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Use explicit boolean values for boolean JSX props.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;input disabled /&gt;
   <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Safe fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Add explicit `true` literal for this attribute</span>
  
<strong>  </strong><strong>  1 │ </strong>&lt;input<span style="opacity: 0.8;">·</span>disabled<span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;">{</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">u</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">}</span><span style="opacity: 0.8;">·</span>/&gt;
<strong>  </strong><strong>    │ </strong>               <span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span>   
</code></pre>{% endraw %}

### Valid

```jsx
<input disabled={false} />
```

```jsx
<input disabled={''} />
```

```jsx
<input disabled={0} />
```

```jsx
<input disabled={undefined} />
```

```jsx
<input disabled='false' />
```

