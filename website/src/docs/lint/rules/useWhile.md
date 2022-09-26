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

{% raw %}<pre class="language-text"><code class="language-text">correctness/useWhile.js:1:1 <a href="https://rome.tools/docs/lint/rules/useWhile">lint/correctness/useWhile</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Use </span><span style="color: Tomato;"><strong>while</strong></span><span style="color: Tomato;"> loops instead of </span><span style="color: Tomato;"><strong>for</strong></span><span style="color: Tomato;"> loops.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>for (; x.running;) {
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>    x.step();
    <strong>3 │ </strong>}
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Use a while loop</span>
  
      | <span style="color: rgb(38, 148, 255);">@@ -1,3 +1,3 @@</span>
  0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">for (; x.running;) {</span>
    0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">while (x.running) {</span>
  1 1 |       x.step();
  2 2 |   }
  
</code></pre>{% endraw %}

