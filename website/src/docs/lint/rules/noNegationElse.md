---
title: Lint Rule noNegationElse
layout: layouts/rule.liquid
---

# noNegationElse (since v0.7.0)

> This rule is recommended by Rome.

Disallow negation in the condition of an `if` statement if it has an `else` clause

## Examples

### Invalid

```jsx
if (!true) {consequent;} else {alternate;}
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noNegationElse/">style/noNegationElse</a></span><span style="color: Tomato;">]</span><em>: </em><em>Invert blocks when performing a negation test.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> style/noNegationElse.js:1:1
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">i</span><span style="color: Tomato;">f</span><span style="color: Tomato;"> </span><span style="color: Tomato;">(</span><span style="color: Tomato;">!</span><span style="color: Tomato;">t</span><span style="color: Tomato;">r</span><span style="color: Tomato;">u</span><span style="color: Tomato;">e</span><span style="color: Tomato;">)</span><span style="color: Tomato;"> </span><span style="color: Tomato;">{</span><span style="color: Tomato;">c</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;">s</span><span style="color: Tomato;">e</span><span style="color: Tomato;">q</span><span style="color: Tomato;">u</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">t</span><span style="color: Tomato;">;</span><span style="color: Tomato;">}</span><span style="color: Tomato;"> </span><span style="color: Tomato;">e</span><span style="color: Tomato;">l</span><span style="color: Tomato;">s</span><span style="color: Tomato;">e</span><span style="color: Tomato;"> </span><span style="color: Tomato;">{</span><span style="color: Tomato;">a</span><span style="color: Tomato;">l</span><span style="color: Tomato;">t</span><span style="color: Tomato;">e</span><span style="color: Tomato;">r</span><span style="color: Tomato;">n</span><span style="color: Tomato;">a</span><span style="color: Tomato;">t</span><span style="color: Tomato;">e</span><span style="color: Tomato;">;</span><span style="color: Tomato;">}</span>
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Exchange alternate and consequent of the node</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">if (!true) {consequent;} else {alternate;}</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">if (true) {alternate;} else {consequent;}</span>

</code></pre>{% endraw %}

```jsx
!true ? consequent : alternate
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noNegationElse/">style/noNegationElse</a></span><span style="color: Tomato;">]</span><em>: </em><em>Invert blocks when performing a negation test.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> style/noNegationElse.js:1:1
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">!</span><span style="color: Tomato;">t</span><span style="color: Tomato;">r</span><span style="color: Tomato;">u</span><span style="color: Tomato;">e</span><span style="color: Tomato;"> </span><span style="color: Tomato;">?</span><span style="color: Tomato;"> </span><span style="color: Tomato;">c</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;">s</span><span style="color: Tomato;">e</span><span style="color: Tomato;">q</span><span style="color: Tomato;">u</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">t</span><span style="color: Tomato;"> </span><span style="color: Tomato;">:</span><span style="color: Tomato;"> </span><span style="color: Tomato;">a</span><span style="color: Tomato;">l</span><span style="color: Tomato;">t</span><span style="color: Tomato;">e</span><span style="color: Tomato;">r</span><span style="color: Tomato;">n</span><span style="color: Tomato;">a</span><span style="color: Tomato;">t</span><span style="color: Tomato;">e</span>
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Exchange alternate and consequent of the node</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">!true ? consequent : alternate</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">true ? alternate : consequent</span>

</code></pre>{% endraw %}

### Valid

```jsx
if (!true) {consequent;}
```

```jsx
true ? consequent : alternate
```

