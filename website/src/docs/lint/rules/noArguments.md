---
title: Lint Rule noArguments
layout: layouts/rule.liquid
---

# noArguments (since v0.7.0)

> This rule is recommended by Rome.

Disallow the use of `arguments`

## Examples

### Invalid

```jsx
function f() {
   console.log(arguments);
}
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noArguments.js:2:16 <a href="https://rome.tools/docs/lint/rules/noArguments">lint/correctness/noArguments</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Use the </span><span style="color: Tomato;"><strong>rest parameters</strong></span><span style="color: Tomato;"> instead of </span><span style="color: Tomato;"><strong>arguments</strong></span><span style="color: Tomato;">.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noArguments.js:2:16
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span>    console.log(<span style="color: Tomato;">a</span><span style="color: Tomato;">r</span><span style="color: Tomato;">g</span><span style="color: Tomato;">u</span><span style="color: Tomato;">m</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">t</span><span style="color: Tomato;">s</span>);
    <span style="color: rgb(38, 148, 255);">│</span>                <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);"><strong>arguments</strong></span><span style="color: rgb(38, 148, 255);"> does not have </span><span style="color: rgb(38, 148, 255);"><strong>Array.prototype</strong></span><span style="color: rgb(38, 148, 255);"> methods and can be inconvenient to use.</span>
  
</code></pre>{% endraw %}

### Valid

```js
function f() {
    let arguments = 1;
    console.log(arguments);
}
```

