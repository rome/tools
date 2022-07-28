---
title: Lint Rule noUnsafeNegation
layout: layouts/rule.liquid
---

# noUnsafeNegation (since v0.7.0)

> This rule is recommended by Rome.

Disallow using unsafe negation.

## Examples

### Invalid

```jsx
!1 in [1,2];
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noUnsafeNegation/">js/noUnsafeNegation</a></span><span style="color: Tomato;">]</span><em>: </em><em>The negation operator is used unsafely on the left side of this binary expression.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noUnsafeNegation.js:1:1
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">!</span><span style="color: Tomato;">1</span><span style="color: Tomato;"> </span><span style="color: Tomato;">i</span><span style="color: Tomato;">n</span><span style="color: Tomato;"> </span><span style="color: Tomato;">[</span><span style="color: Tomato;">1</span><span style="color: Tomato;">,</span><span style="color: Tomato;">2</span><span style="color: Tomato;">]</span>;
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Wrap the expression with a parenthesis</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">!1 in [1,2];</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">!(1 in [1,2]);</span>

</code></pre>{% endraw %}

```jsx
/**test*/!/** test*/1 instanceof [1,2];
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noUnsafeNegation/">js/noUnsafeNegation</a></span><span style="color: Tomato;">]</span><em>: </em><em>The negation operator is used unsafely on the left side of this binary expression.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noUnsafeNegation.js:1:10
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> /**test*/<span style="color: Tomato;">!</span><span style="color: Tomato;">/</span><span style="color: Tomato;">*</span><span style="color: Tomato;">*</span><span style="color: Tomato;"> </span><span style="color: Tomato;">t</span><span style="color: Tomato;">e</span><span style="color: Tomato;">s</span><span style="color: Tomato;">t</span><span style="color: Tomato;">*</span><span style="color: Tomato;">/</span><span style="color: Tomato;">1</span><span style="color: Tomato;"> </span><span style="color: Tomato;">i</span><span style="color: Tomato;">n</span><span style="color: Tomato;">s</span><span style="color: Tomato;">t</span><span style="color: Tomato;">a</span><span style="color: Tomato;">n</span><span style="color: Tomato;">c</span><span style="color: Tomato;">e</span><span style="color: Tomato;">o</span><span style="color: Tomato;">f</span><span style="color: Tomato;"> </span><span style="color: Tomato;">[</span><span style="color: Tomato;">1</span><span style="color: Tomato;">,</span><span style="color: Tomato;">2</span><span style="color: Tomato;">]</span>;
  <span style="color: rgb(38, 148, 255);">│</span>          <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Wrap the expression with a parenthesis</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">/**test*/!/** test*/1 instanceof [1,2];</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">/**test*/!/** test*/(1 instanceof [1,2]);</span>

</code></pre>{% endraw %}

### Valid

```jsx
-1 in [1,2];
~1 in [1,2];
typeof 1 in [1,2];
void 1 in [1,2];
delete 1 in [1,2];
+1 instanceof [1,2];
```

