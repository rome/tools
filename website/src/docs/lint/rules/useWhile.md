---
title: Lint Rule useWhile
layout: layouts/rule.liquid
---

# useWhile

Enforce the use of `while` loops instead of `for` loops when the
initializer and update expressions are not needed

## Examples

### Invalid

```jsx
for (; x.running;) {
    x.step();
}
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;">useWhile</span><span style="color: Orange;">]</span><em>: </em><em>Use </em><em><em>while</em></em><em> loops instead of </em><em><em>for</em></em><em> loops.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> useWhile.js:1:1
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> for (; x.running;) {
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Use a while loop</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1,3 +1,3 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">for (; x.running;) {</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">while (x.running) {</span>
1 1 |       x.step();
2 2 |   }

</code></pre>{% endraw %}

