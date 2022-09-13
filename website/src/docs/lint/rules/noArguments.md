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

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noArguments/">correctness/noArguments</a></span><span style="color: Tomato;">]</span><em>: </em><em>Use the </em><em><em>rest parameters</em></em><em> instead of </em><em><em>arguments</em></em><em>.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noArguments.js:2:16
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span>    console.log(<span style="color: Tomato;">a</span><span style="color: Tomato;">r</span><span style="color: Tomato;">g</span><span style="color: Tomato;">u</span><span style="color: Tomato;">m</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">t</span><span style="color: Tomato;">s</span>);
  <span style="color: rgb(38, 148, 255);">│</span>                <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

=  note: <em>arguments</em> does not have <em>Array.prototype</em> methods and can be inconvenient to use.

</code></pre>{% endraw %}

### Valid

```js
function f() {
    let arguments = 1;
    console.log(arguments);
}
```

