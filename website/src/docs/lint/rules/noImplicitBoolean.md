---
title: Lint Rule noImplicitBoolean
layout: layouts/rule.liquid
---

# noImplicitBoolean (since v0.7.0)

> This rule is recommended by Rome.

Disallow implicit `true` values on JSX boolean attributes

## Examples

### Invalid

```jsx
<input disabled />
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noImplicitBoolean/">jsx/noImplicitBoolean</a></span><span style="color: Orange;">]</span><em>: </em><em>Use explicit boolean values for boolean JSX props.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> jsx/noImplicitBoolean.js:1:8
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> &lt;input disabled /&gt;
  <span style="color: rgb(38, 148, 255);">│</span>        <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

<span style="color: rgb(38, 148, 255);">Safe fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Add explicit `true` literal for this attribute</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">&lt;input disabled /&gt;</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">&lt;input disabled={true} /&gt;</span>

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

