---
title: Lint Rule useShorthandArrayType
layout: layouts/rule.liquid
---

# useShorthandArrayType

Enforce the use of `while` loops instead of `for` loops when the

## Examples

### Valid

```jsx
let valid: Array<Foo | Bar>;
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;">SyntaxError</span><span style="color: Tomato;">]</span><em>: </em><em>type annotation are a TypeScript only feature. Convert your file to a TypeScript file or remove the syntax.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> useShorthandArrayType.js:1:10
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> let valid<span style="color: Tomato;">:</span><span style="color: Tomato;"> </span><span style="color: Tomato;">A</span><span style="color: Tomato;">r</span><span style="color: Tomato;">r</span><span style="color: Tomato;">a</span><span style="color: Tomato;">y</span><span style="color: Tomato;">&lt;</span><span style="color: Tomato;">F</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span><span style="color: Tomato;"> </span><span style="color: Tomato;">|</span><span style="color: Tomato;"> </span><span style="color: Tomato;">B</span><span style="color: Tomato;">a</span><span style="color: Tomato;">r</span><span style="color: Tomato;">&gt;</span>;
  <span style="color: rgb(38, 148, 255);">│</span>          <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span> <span style="color: Tomato;">TypeScript only syntax</span>

</code></pre>{% endraw %}

