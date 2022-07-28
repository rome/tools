---
title: Lint Rule useWhile
layout: layouts/rule.liquid
---

# useWhile (since v0.7.0)

> This rule is recommended by Rome.

Enforce the use of `while` loops instead of `for` loops when the
initializer and update expressions are not needed

## Examples

### Invalid

```jsx
for (; x.running;) {
    x.step();
}
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/useWhile/">js/useWhile</a></span><span style="color: Tomato;">]</span><em>: </em><em>Use </em><em><em>while</em></em><em> loops instead of </em><em><em>for</em></em><em> loops.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/useWhile.js:1:1
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">r</span><span style="color: Tomato;"> </span><span style="color: Tomato;">(</span><span style="color: Tomato;">;</span><span style="color: Tomato;"> </span><span style="color: Tomato;">x</span><span style="color: Tomato;">.</span><span style="color: Tomato;">r</span><span style="color: Tomato;">u</span><span style="color: Tomato;">n</span><span style="color: Tomato;">n</span><span style="color: Tomato;">i</span><span style="color: Tomato;">n</span><span style="color: Tomato;">g</span><span style="color: Tomato;">;</span><span style="color: Tomato;">)</span> {
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Use a while loop</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1,3 +1,3 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">for (; x.running;) {</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">while (x.running) {</span>
1 1 |       x.step();
2 2 |   }

</code></pre>{% endraw %}

