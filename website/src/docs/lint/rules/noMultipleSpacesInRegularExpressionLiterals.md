---
title: Lint Rule noMultipleSpacesInRegularExpressionLiterals
layout: layouts/rule.liquid
---

# noMultipleSpacesInRegularExpressionLiterals (since v0.7.0)

> This rule is recommended by Rome.

Disallow unclear usage of multiple space characters in regular expression literals

## Examples

### Invalid

```jsx
/   /
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noMultipleSpacesInRegularExpressionLiterals/">correctness/noMultipleSpacesInRegularExpressionLiterals</a></span><span style="color: Tomato;">]</span><em>: </em><em>This regular expression contains unclear uses of multiple spaces.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noMultipleSpacesInRegularExpressionLiterals.js:1:2
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> /<span style="color: Tomato;"> </span><span style="color: Tomato;"> </span><span style="color: Tomato;"> </span>/
  <span style="color: rgb(38, 148, 255);">│</span>  <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">It's hard to visually count the amount of spaces, it's clearer if you use a quantifier instead. eg / {3}/</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">/   /</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">/ {3}/</span>

</code></pre>{% endraw %}

```jsx
/  foo/
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noMultipleSpacesInRegularExpressionLiterals/">correctness/noMultipleSpacesInRegularExpressionLiterals</a></span><span style="color: Tomato;">]</span><em>: </em><em>This regular expression contains unclear uses of multiple spaces.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noMultipleSpacesInRegularExpressionLiterals.js:1:2
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> /<span style="color: Tomato;"> </span><span style="color: Tomato;"> </span>foo/
  <span style="color: rgb(38, 148, 255);">│</span>  <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">It's hard to visually count the amount of spaces, it's clearer if you use a quantifier instead. eg / {2}/</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">/  foo/</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">/ {2}foo/</span>

</code></pre>{% endraw %}

```jsx
/foo   /
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noMultipleSpacesInRegularExpressionLiterals/">correctness/noMultipleSpacesInRegularExpressionLiterals</a></span><span style="color: Tomato;">]</span><em>: </em><em>This regular expression contains unclear uses of multiple spaces.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noMultipleSpacesInRegularExpressionLiterals.js:1:5
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> /foo<span style="color: Tomato;"> </span><span style="color: Tomato;"> </span><span style="color: Tomato;"> </span>/
  <span style="color: rgb(38, 148, 255);">│</span>     <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">It's hard to visually count the amount of spaces, it's clearer if you use a quantifier instead. eg / {3}/</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">/foo   /</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">/foo {3}/</span>

</code></pre>{% endraw %}

```jsx
/foo  bar/
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noMultipleSpacesInRegularExpressionLiterals/">correctness/noMultipleSpacesInRegularExpressionLiterals</a></span><span style="color: Tomato;">]</span><em>: </em><em>This regular expression contains unclear uses of multiple spaces.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noMultipleSpacesInRegularExpressionLiterals.js:1:5
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> /foo<span style="color: Tomato;"> </span><span style="color: Tomato;"> </span>bar/
  <span style="color: rgb(38, 148, 255);">│</span>     <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">It's hard to visually count the amount of spaces, it's clearer if you use a quantifier instead. eg / {2}/</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">/foo  bar/</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">/foo {2}bar/</span>

</code></pre>{% endraw %}

```jsx
/foo   bar    baz/
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noMultipleSpacesInRegularExpressionLiterals/">correctness/noMultipleSpacesInRegularExpressionLiterals</a></span><span style="color: Tomato;">]</span><em>: </em><em>This regular expression contains unclear uses of multiple spaces.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noMultipleSpacesInRegularExpressionLiterals.js:1:5
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> /foo<span style="color: Tomato;"> </span><span style="color: Tomato;"> </span><span style="color: Tomato;"> </span><span style="color: Tomato;">b</span><span style="color: Tomato;">a</span><span style="color: Tomato;">r</span><span style="color: Tomato;"> </span><span style="color: Tomato;"> </span><span style="color: Tomato;"> </span><span style="color: Tomato;"> </span>baz/
  <span style="color: rgb(38, 148, 255);">│</span>     <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">It's hard to visually count the amount of spaces, it's clearer if you use a quantifier instead. eg / {7}/</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">/foo   bar    baz/</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">/foo {3}bar {4}baz/</span>

</code></pre>{% endraw %}

```jsx
/foo [ba]r  b(a|z)/
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noMultipleSpacesInRegularExpressionLiterals/">correctness/noMultipleSpacesInRegularExpressionLiterals</a></span><span style="color: Tomato;">]</span><em>: </em><em>This regular expression contains unclear uses of multiple spaces.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noMultipleSpacesInRegularExpressionLiterals.js:1:11
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> /foo [ba]r<span style="color: Tomato;"> </span><span style="color: Tomato;"> </span>b(a|z)/
  <span style="color: rgb(38, 148, 255);">│</span>           <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">It's hard to visually count the amount of spaces, it's clearer if you use a quantifier instead. eg / {2}/</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">/foo [ba]r  b(a|z)/</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">/foo [ba]r {2}b(a|z)/</span>

</code></pre>{% endraw %}

### Valid

```jsx
/foo {2}bar/
```

```jsx
/foo bar baz/
```

```jsx
/foo bar	baz/
```

```jsx
/foo /
```

