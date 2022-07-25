---
title: Lint Rule noMultipleSpacesInRegularExpressionLiterals
layout: layouts/rule.liquid
---

# noMultipleSpacesInRegularExpressionLiterals (since v0.7.0)

Disallow unclear usage of multiple space characters in regular expression literals

## Examples

### Invalid

```jsx
/   /
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noMultipleSpacesInRegularExpressionLiterals/">regex/noMultipleSpacesInRegularExpressionLiterals</a></span><span style="color: Orange;">]</span><em>: </em><em>This regular expression contains unclear uses of multiple spaces.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> regex/noMultipleSpacesInRegularExpressionLiterals.js:1:2
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> /   /
  <span style="color: rgb(38, 148, 255);">│</span>  <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">It's hard to visually count the amount of spaces, it's clearer if you use a quantifier instead. eg / {3}/</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">/   /</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">/ {3}/</span>

</code></pre>{% endraw %}

```jsx
/  foo/
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noMultipleSpacesInRegularExpressionLiterals/">regex/noMultipleSpacesInRegularExpressionLiterals</a></span><span style="color: Orange;">]</span><em>: </em><em>This regular expression contains unclear uses of multiple spaces.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> regex/noMultipleSpacesInRegularExpressionLiterals.js:1:2
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> /  foo/
  <span style="color: rgb(38, 148, 255);">│</span>  <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">It's hard to visually count the amount of spaces, it's clearer if you use a quantifier instead. eg / {2}/</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">/  foo/</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">/ {2}foo/</span>

</code></pre>{% endraw %}

```jsx
/foo   /
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noMultipleSpacesInRegularExpressionLiterals/">regex/noMultipleSpacesInRegularExpressionLiterals</a></span><span style="color: Orange;">]</span><em>: </em><em>This regular expression contains unclear uses of multiple spaces.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> regex/noMultipleSpacesInRegularExpressionLiterals.js:1:5
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> /foo   /
  <span style="color: rgb(38, 148, 255);">│</span>     <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">It's hard to visually count the amount of spaces, it's clearer if you use a quantifier instead. eg / {3}/</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">/foo   /</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">/foo {3}/</span>

</code></pre>{% endraw %}

```jsx
/foo  bar/
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noMultipleSpacesInRegularExpressionLiterals/">regex/noMultipleSpacesInRegularExpressionLiterals</a></span><span style="color: Orange;">]</span><em>: </em><em>This regular expression contains unclear uses of multiple spaces.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> regex/noMultipleSpacesInRegularExpressionLiterals.js:1:5
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> /foo  bar/
  <span style="color: rgb(38, 148, 255);">│</span>     <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">It's hard to visually count the amount of spaces, it's clearer if you use a quantifier instead. eg / {2}/</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">/foo  bar/</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">/foo {2}bar/</span>

</code></pre>{% endraw %}

```jsx
/foo   bar    baz/
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noMultipleSpacesInRegularExpressionLiterals/">regex/noMultipleSpacesInRegularExpressionLiterals</a></span><span style="color: Orange;">]</span><em>: </em><em>This regular expression contains unclear uses of multiple spaces.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> regex/noMultipleSpacesInRegularExpressionLiterals.js:1:5
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> /foo   bar    baz/
  <span style="color: rgb(38, 148, 255);">│</span>     <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">It's hard to visually count the amount of spaces, it's clearer if you use a quantifier instead. eg / {7}/</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">/foo   bar    baz/</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">/foo {3}bar {4}baz/</span>

</code></pre>{% endraw %}

```jsx
/foo [ba]r  b(a|z)/
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noMultipleSpacesInRegularExpressionLiterals/">regex/noMultipleSpacesInRegularExpressionLiterals</a></span><span style="color: Orange;">]</span><em>: </em><em>This regular expression contains unclear uses of multiple spaces.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> regex/noMultipleSpacesInRegularExpressionLiterals.js:1:11
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> /foo [ba]r  b(a|z)/
  <span style="color: rgb(38, 148, 255);">│</span>           <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

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

