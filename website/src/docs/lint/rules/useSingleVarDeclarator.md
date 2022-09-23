---
title: Lint Rule useSingleVarDeclarator
layout: layouts/rule.liquid
---

# useSingleVarDeclarator (since v0.7.0)

> This rule is recommended by Rome.

Disallow multiple variable declarations in the same variable statement

## Examples

### Invalid

```jsx
let foo, bar;
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/useSingleVarDeclarator.js:1:1 <a href="https://rome.tools/docs/lint/rules/useSingleVarDeclarator">lint/correctness/useSingleVarDeclarator</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Declare variables separately</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/useSingleVarDeclarator.js:1:1
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">l</span><span style="color: Tomato;">e</span><span style="color: Tomato;">t</span><span style="color: Tomato;"> </span><span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span><span style="color: Tomato;">,</span><span style="color: Tomato;"> </span><span style="color: Tomato;">b</span><span style="color: Tomato;">a</span><span style="color: Tomato;">r</span><span style="color: Tomato;">;</span>
    <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Break out into multiple declarations</span>
  
      | <span style="color: rgb(38, 148, 255);">@@ -1 +1,2 @@</span>
  0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">let foo, bar;</span>
    0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">let foo;</span>
    1 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">let bar;</span>
  
</code></pre>{% endraw %}

### Valid

```jsx
for (let i = 0, x = 1; i < arr.length; i++) {}
```

