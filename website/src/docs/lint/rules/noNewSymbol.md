---
title: Lint Rule noNewSymbol
layout: layouts/rule.liquid
---

# noNewSymbol (since v0.10.0)

> This rule is recommended by Rome.

Disallow `new` operators with the `Symbol` object

## Examples

### Invalid

```jsx
var foo = new Symbol('foo');
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noNewSymbol/">nursery/noNewSymbol</a></span><span style="color: Tomato;">]</span><em>: </em><em><em>Symbol</em></em><em> cannot be called as a constructor.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noNewSymbol.js:1:11
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> var foo = <span style="color: Tomato;">n</span><span style="color: Tomato;">e</span><span style="color: Tomato;">w</span><span style="color: Tomato;"> </span><span style="color: Tomato;">S</span><span style="color: Tomato;">y</span><span style="color: Tomato;">m</span><span style="color: Tomato;">b</span><span style="color: Tomato;">o</span><span style="color: Tomato;">l</span><span style="color: Tomato;">(</span><span style="color: Tomato;">'</span><span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span><span style="color: Tomato;">'</span><span style="color: Tomato;">)</span>;
  <span style="color: rgb(38, 148, 255);">│</span>           <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove </span><span style="color: rgb(38, 148, 255);"><em>new</em></span><span style="color: rgb(38, 148, 255);">.</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">var foo = new Symbol('foo');</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">var foo = Symbol('foo');</span>

</code></pre>{% endraw %}

### Valid

```jsx
var bar = Symbol('bar');
function baz() {
    function Symbol() { }
    new Symbol();
}
```

