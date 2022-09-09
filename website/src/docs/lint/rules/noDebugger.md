---
title: Lint Rule noDebugger
layout: layouts/rule.liquid
---

# noDebugger (since v0.7.0)

> This rule is recommended by Rome.

Disallow the use of `debugger`

## Examples

### Invalid

```jsx
debugger;
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noDebugger/">correctness/noDebugger</a></span><span style="color: Tomato;">]</span><em>: </em><em>This is an unexpected use of the </em><em><em>debugger</em></em><em> statement.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noDebugger.js:1:1
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">d</span><span style="color: Tomato;">e</span><span style="color: Tomato;">b</span><span style="color: Tomato;">u</span><span style="color: Tomato;">g</span><span style="color: Tomato;">g</span><span style="color: Tomato;">e</span><span style="color: Tomato;">r</span><span style="color: Tomato;">;</span>
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove debugger statement</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">debugger;</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;"></span>

</code></pre>{% endraw %}

### Valid

```jsx
const test = { debugger: 1 };
test.debugger;
```

