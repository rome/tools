---
title: Lint Rule useTemplate
layout: layouts/rule.liquid
---

# useTemplate (since v0.7.0)

> This rule is recommended by Rome.

Template literals are preferred over string concatenation.

## Examples

### Invalid

```jsx
console.log(foo + "baz");
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/useTemplate/">correctness/useTemplate</a></span><span style="color: Tomato;">]</span><strong>: </strong><strong></strong><strong><strong>Template</strong></strong><strong> literals are preferred over </strong><strong><strong>string concatenation.</strong></strong><strong></strong>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/useTemplate.js:1:13
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> console.log(<span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span><span style="color: Tomato;"> </span><span style="color: Tomato;">+</span><span style="color: Tomato;"> </span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">b</span><span style="color: Tomato;">a</span><span style="color: Tomato;">z</span><span style="color: Tomato;">&quot;</span>);
  <span style="color: rgb(38, 148, 255);">│</span>             <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Use a </span><span style="color: rgb(38, 148, 255);"><strong>TemplateLiteral</strong></span><span style="color: rgb(38, 148, 255);">.</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">console.log(foo + &quot;baz&quot;);</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">console.log(`${foo}baz`);</span>

</code></pre>{% endraw %}

```jsx
console.log(1 * 2 + "foo");
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/useTemplate/">correctness/useTemplate</a></span><span style="color: Tomato;">]</span><strong>: </strong><strong></strong><strong><strong>Template</strong></strong><strong> literals are preferred over </strong><strong><strong>string concatenation.</strong></strong><strong></strong>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/useTemplate.js:1:13
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> console.log(<span style="color: Tomato;">1</span><span style="color: Tomato;"> </span><span style="color: Tomato;">*</span><span style="color: Tomato;"> </span><span style="color: Tomato;">2</span><span style="color: Tomato;"> </span><span style="color: Tomato;">+</span><span style="color: Tomato;"> </span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span><span style="color: Tomato;">&quot;</span>);
  <span style="color: rgb(38, 148, 255);">│</span>             <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Use a </span><span style="color: rgb(38, 148, 255);"><strong>TemplateLiteral</strong></span><span style="color: rgb(38, 148, 255);">.</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">console.log(1 * 2 + &quot;foo&quot;);</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">console.log(`${1 * 2}foo`);</span>

</code></pre>{% endraw %}

```jsx
console.log(1 + "foo" + 2 + "bar" + "baz" + 3);
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/useTemplate/">correctness/useTemplate</a></span><span style="color: Tomato;">]</span><strong>: </strong><strong></strong><strong><strong>Template</strong></strong><strong> literals are preferred over </strong><strong><strong>string concatenation.</strong></strong><strong></strong>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/useTemplate.js:1:13
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> console.log(<span style="color: Tomato;">1</span><span style="color: Tomato;"> </span><span style="color: Tomato;">+</span><span style="color: Tomato;"> </span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;"> </span><span style="color: Tomato;">+</span><span style="color: Tomato;"> </span><span style="color: Tomato;">2</span><span style="color: Tomato;"> </span><span style="color: Tomato;">+</span><span style="color: Tomato;"> </span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">b</span><span style="color: Tomato;">a</span><span style="color: Tomato;">r</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;"> </span><span style="color: Tomato;">+</span><span style="color: Tomato;"> </span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">b</span><span style="color: Tomato;">a</span><span style="color: Tomato;">z</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;"> </span><span style="color: Tomato;">+</span><span style="color: Tomato;"> </span><span style="color: Tomato;">3</span>);
  <span style="color: rgb(38, 148, 255);">│</span>             <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Use a </span><span style="color: rgb(38, 148, 255);"><strong>TemplateLiteral</strong></span><span style="color: rgb(38, 148, 255);">.</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">console.log(1 + &quot;foo&quot; + 2 + &quot;bar&quot; + &quot;baz&quot; + 3);</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">console.log(`${1}foo${2}barbaz${3}`);</span>

</code></pre>{% endraw %}

```jsx
console.log((1 + "foo") * 2);
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/useTemplate/">correctness/useTemplate</a></span><span style="color: Tomato;">]</span><strong>: </strong><strong></strong><strong><strong>Template</strong></strong><strong> literals are preferred over </strong><strong><strong>string concatenation.</strong></strong><strong></strong>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/useTemplate.js:1:14
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> console.log((<span style="color: Tomato;">1</span><span style="color: Tomato;"> </span><span style="color: Tomato;">+</span><span style="color: Tomato;"> </span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span><span style="color: Tomato;">&quot;</span>) * 2);
  <span style="color: rgb(38, 148, 255);">│</span>              <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Use a </span><span style="color: rgb(38, 148, 255);"><strong>TemplateLiteral</strong></span><span style="color: rgb(38, 148, 255);">.</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">console.log((1 + &quot;foo&quot;) * 2);</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">console.log((`${1}foo`) * 2);</span>

</code></pre>{% endraw %}

```jsx
console.log("foo" + 1);
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/useTemplate/">correctness/useTemplate</a></span><span style="color: Tomato;">]</span><strong>: </strong><strong></strong><strong><strong>Template</strong></strong><strong> literals are preferred over </strong><strong><strong>string concatenation.</strong></strong><strong></strong>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/useTemplate.js:1:13
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> console.log(<span style="color: Tomato;">&quot;</span><span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;"> </span><span style="color: Tomato;">+</span><span style="color: Tomato;"> </span><span style="color: Tomato;">1</span>);
  <span style="color: rgb(38, 148, 255);">│</span>             <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Use a </span><span style="color: rgb(38, 148, 255);"><strong>TemplateLiteral</strong></span><span style="color: rgb(38, 148, 255);">.</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">console.log(&quot;foo&quot; + 1);</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">console.log(`foo${1}`);</span>

</code></pre>{% endraw %}

### Valid

```jsx
console.log("foo" + "bar");
console.log(foo() + "\n");
```

