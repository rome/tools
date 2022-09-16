---
title: Lint Rule useBlockStatements
layout: layouts/rule.liquid
---

# useBlockStatements (since v0.7.0)

> This rule is recommended by Rome.

Requires following curly brace conventions.
JavaScript allows the omission of curly braces when a block contains only one statement. However, it is considered by many to be best practice to never omit curly braces around blocks, even when they are optional, because it can lead to bugs and reduces code clarity.

## Examples

### Invalid

```jsx
 if (x) x;
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/useBlockStatements/">correctness/useBlockStatements</a></span><span style="color: Tomato;">]</span><strong>: </strong><strong>Block statements are preferred in this position.</strong>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/useBlockStatements.js:1:2
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span>  <span style="color: Tomato;">i</span><span style="color: Tomato;">f</span><span style="color: Tomato;"> </span><span style="color: Tomato;">(</span><span style="color: Tomato;">x</span><span style="color: Tomato;">)</span><span style="color: Tomato;"> </span><span style="color: Tomato;">x</span><span style="color: Tomato;">;</span>
  <span style="color: rgb(38, 148, 255);">│</span>  <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Wrap the statement with a `JsBlockStatement`</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;"> if (x) x;</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;"> if (x) { x; }</span>

</code></pre>{% endraw %}

```jsx
 if (x) {
   x;
 } else y;
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/useBlockStatements/">correctness/useBlockStatements</a></span><span style="color: Tomato;">]</span><strong>: </strong><strong>Block statements are preferred in this position.</strong>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/useBlockStatements.js:3:4
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">3</span> <span style="color: rgb(38, 148, 255);">│</span>  } <span style="color: Tomato;">e</span><span style="color: Tomato;">l</span><span style="color: Tomato;">s</span><span style="color: Tomato;">e</span><span style="color: Tomato;"> </span><span style="color: Tomato;">y</span><span style="color: Tomato;">;</span>
  <span style="color: rgb(38, 148, 255);">│</span>    <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Wrap the statement with a `JsBlockStatement`</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1,3 +1,3 @@</span>
0 0 |    if (x) {
1 1 |      x;
2   | <span style="color: Tomato;">- </span><span style="color: Tomato;"> } else y;</span>
  2 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;"> } else { y; }</span>

</code></pre>{% endraw %}

```jsx
if (x) {
  x;
} else if (y) y;
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/useBlockStatements/">correctness/useBlockStatements</a></span><span style="color: Tomato;">]</span><strong>: </strong><strong>Block statements are preferred in this position.</strong>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/useBlockStatements.js:3:8
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">3</span> <span style="color: rgb(38, 148, 255);">│</span> } else <span style="color: Tomato;">i</span><span style="color: Tomato;">f</span><span style="color: Tomato;"> </span><span style="color: Tomato;">(</span><span style="color: Tomato;">y</span><span style="color: Tomato;">)</span><span style="color: Tomato;"> </span><span style="color: Tomato;">y</span><span style="color: Tomato;">;</span>
  <span style="color: rgb(38, 148, 255);">│</span>        <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Wrap the statement with a `JsBlockStatement`</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1,3 +1,3 @@</span>
0 0 |   if (x) {
1 1 |     x;
2   | <span style="color: Tomato;">- </span><span style="color: Tomato;">} else if (y) y;</span>
  2 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">} else if (y) { y; }</span>

</code></pre>{% endraw %}

```jsx
   for (;;);
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/useBlockStatements/">correctness/useBlockStatements</a></span><span style="color: Tomato;">]</span><strong>: </strong><strong>Block statements are preferred in this position.</strong>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/useBlockStatements.js:1:4
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span>    <span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">r</span><span style="color: Tomato;"> </span><span style="color: Tomato;">(</span><span style="color: Tomato;">;</span><span style="color: Tomato;">;</span><span style="color: Tomato;">)</span><span style="color: Tomato;">;</span>
  <span style="color: rgb(38, 148, 255);">│</span>    <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Wrap the statement with a `JsBlockStatement`</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">   for (;;);</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">   for (;;) {}</span>

</code></pre>{% endraw %}

```jsx
   for (p in obj);
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/useBlockStatements/">correctness/useBlockStatements</a></span><span style="color: Tomato;">]</span><strong>: </strong><strong>Block statements are preferred in this position.</strong>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/useBlockStatements.js:1:4
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span>    <span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">r</span><span style="color: Tomato;"> </span><span style="color: Tomato;">(</span><span style="color: Tomato;">p</span><span style="color: Tomato;"> </span><span style="color: Tomato;">i</span><span style="color: Tomato;">n</span><span style="color: Tomato;"> </span><span style="color: Tomato;">o</span><span style="color: Tomato;">b</span><span style="color: Tomato;">j</span><span style="color: Tomato;">)</span><span style="color: Tomato;">;</span>
  <span style="color: rgb(38, 148, 255);">│</span>    <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Wrap the statement with a `JsBlockStatement`</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">   for (p in obj);</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">   for (p in obj) {}</span>

</code></pre>{% endraw %}

```jsx
  for (x of xs);
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/useBlockStatements/">correctness/useBlockStatements</a></span><span style="color: Tomato;">]</span><strong>: </strong><strong>Block statements are preferred in this position.</strong>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/useBlockStatements.js:1:3
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span>   <span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">r</span><span style="color: Tomato;"> </span><span style="color: Tomato;">(</span><span style="color: Tomato;">x</span><span style="color: Tomato;"> </span><span style="color: Tomato;">o</span><span style="color: Tomato;">f</span><span style="color: Tomato;"> </span><span style="color: Tomato;">x</span><span style="color: Tomato;">s</span><span style="color: Tomato;">)</span><span style="color: Tomato;">;</span>
  <span style="color: rgb(38, 148, 255);">│</span>   <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Wrap the statement with a `JsBlockStatement`</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">  for (x of xs);</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">  for (x of xs) {}</span>

</code></pre>{% endraw %}

```jsx
  do;
  while (x);
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/useBlockStatements/">correctness/useBlockStatements</a></span><span style="color: Tomato;">]</span><strong>: </strong><strong>Block statements are preferred in this position.</strong>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/useBlockStatements.js:1:3
  <span style="color: rgb(38, 148, 255);">│</span>  
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">┌</span>   <span style="color: Tomato;">d</span><span style="color: Tomato;">o</span><span style="color: Tomato;">;</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">│</span> <span style="color: Tomato;"> </span><span style="color: Tomato;"> </span><span style="color: Tomato;">w</span><span style="color: Tomato;">h</span><span style="color: Tomato;">i</span><span style="color: Tomato;">l</span><span style="color: Tomato;">e</span><span style="color: Tomato;"> </span><span style="color: Tomato;">(</span><span style="color: Tomato;">x</span><span style="color: Tomato;">)</span><span style="color: Tomato;">;</span>
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">└</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Wrap the statement with a `JsBlockStatement`</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1,2 +1,2 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">  do;</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">  do {}</span>
1 1 |     while (x);

</code></pre>{% endraw %}

```jsx
   while (x);
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/useBlockStatements/">correctness/useBlockStatements</a></span><span style="color: Tomato;">]</span><strong>: </strong><strong>Block statements are preferred in this position.</strong>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/useBlockStatements.js:1:4
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span>    <span style="color: Tomato;">w</span><span style="color: Tomato;">h</span><span style="color: Tomato;">i</span><span style="color: Tomato;">l</span><span style="color: Tomato;">e</span><span style="color: Tomato;"> </span><span style="color: Tomato;">(</span><span style="color: Tomato;">x</span><span style="color: Tomato;">)</span><span style="color: Tomato;">;</span>
  <span style="color: rgb(38, 148, 255);">│</span>    <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Wrap the statement with a `JsBlockStatement`</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">   while (x);</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">   while (x) {}</span>

</code></pre>{% endraw %}

```jsx
  with (x);
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;">SyntaxError</span><span style="color: Tomato;">]</span><strong>: </strong><strong>`with` statements are not allowed in strict mode</strong>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/useBlockStatements.js:1:3
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span>   <span style="color: Tomato;">w</span><span style="color: Tomato;">i</span><span style="color: Tomato;">t</span><span style="color: Tomato;">h</span><span style="color: Tomato;"> </span><span style="color: Tomato;">(</span><span style="color: Tomato;">x</span><span style="color: Tomato;">)</span><span style="color: Tomato;">;</span>
  <span style="color: rgb(38, 148, 255);">│</span>   <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

</code></pre>{% endraw %}

