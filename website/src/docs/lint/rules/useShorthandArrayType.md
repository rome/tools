---
title: Lint Rule useShorthandArrayType
layout: layouts/rule.liquid
---

# useShorthandArrayType (since v0.7.0)

> This rule is recommended by Rome.

When expressing array types, this rule promotes the usage of `T[]` shorthand instead of `Array<T>`.

## Examples

### Invalid

```ts
let valid: Array<foo>;
```

{% raw %}<pre class="language-text"><code class="language-text">style/useShorthandArrayType.js:1:12 <a href="https://rome.tools/docs/lint/rules/useShorthandArrayType">lint/style/useShorthandArrayType</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Use </span><span style="color: Tomato;"><strong>shorthand T[] syntax</strong></span><span style="color: Tomato;"> instead of </span><span style="color: Tomato;"><strong>Array&lt;T&gt; syntax.</strong></span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> style/useShorthandArrayType.js:1:12
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> let valid: <span style="color: Tomato;">A</span><span style="color: Tomato;">r</span><span style="color: Tomato;">r</span><span style="color: Tomato;">a</span><span style="color: Tomato;">y</span><span style="color: Tomato;">&lt;</span><span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span><span style="color: Tomato;">&gt;</span>;
    <span style="color: rgb(38, 148, 255);">│</span>            <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Use </span><span style="color: rgb(38, 148, 255);"><strong>shorthand T[] syntax</strong></span><span style="color: rgb(38, 148, 255);"> to replace</span>
  
      | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
  0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">let valid: Array&lt;foo&gt;;</span>
    0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">let valid: foo[];</span>
  
</code></pre>{% endraw %}

```ts
let invalid2: Promise<Array<string>>;
```

{% raw %}<pre class="language-text"><code class="language-text">style/useShorthandArrayType.js:1:23 <a href="https://rome.tools/docs/lint/rules/useShorthandArrayType">lint/style/useShorthandArrayType</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Use </span><span style="color: Tomato;"><strong>shorthand T[] syntax</strong></span><span style="color: Tomato;"> instead of </span><span style="color: Tomato;"><strong>Array&lt;T&gt; syntax.</strong></span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> style/useShorthandArrayType.js:1:23
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> let invalid2: Promise&lt;<span style="color: Tomato;">A</span><span style="color: Tomato;">r</span><span style="color: Tomato;">r</span><span style="color: Tomato;">a</span><span style="color: Tomato;">y</span><span style="color: Tomato;">&lt;</span><span style="color: Tomato;">s</span><span style="color: Tomato;">t</span><span style="color: Tomato;">r</span><span style="color: Tomato;">i</span><span style="color: Tomato;">n</span><span style="color: Tomato;">g</span><span style="color: Tomato;">&gt;</span>&gt;;
    <span style="color: rgb(38, 148, 255);">│</span>                       <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Use </span><span style="color: rgb(38, 148, 255);"><strong>shorthand T[] syntax</strong></span><span style="color: rgb(38, 148, 255);"> to replace</span>
  
      | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
  0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">let invalid2: Promise&lt;Array&lt;string&gt;&gt;;</span>
    0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">let invalid2: Promise&lt;string[]&gt;;</span>
  
</code></pre>{% endraw %}

```ts
let invalid3: Array<Foo<Bar>>;
```

{% raw %}<pre class="language-text"><code class="language-text">style/useShorthandArrayType.js:1:15 <a href="https://rome.tools/docs/lint/rules/useShorthandArrayType">lint/style/useShorthandArrayType</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Use </span><span style="color: Tomato;"><strong>shorthand T[] syntax</strong></span><span style="color: Tomato;"> instead of </span><span style="color: Tomato;"><strong>Array&lt;T&gt; syntax.</strong></span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> style/useShorthandArrayType.js:1:15
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> let invalid3: <span style="color: Tomato;">A</span><span style="color: Tomato;">r</span><span style="color: Tomato;">r</span><span style="color: Tomato;">a</span><span style="color: Tomato;">y</span><span style="color: Tomato;">&lt;</span><span style="color: Tomato;">F</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span><span style="color: Tomato;">&lt;</span><span style="color: Tomato;">B</span><span style="color: Tomato;">a</span><span style="color: Tomato;">r</span><span style="color: Tomato;">&gt;</span><span style="color: Tomato;">&gt;</span>;
    <span style="color: rgb(38, 148, 255);">│</span>               <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Use </span><span style="color: rgb(38, 148, 255);"><strong>shorthand T[] syntax</strong></span><span style="color: rgb(38, 148, 255);"> to replace</span>
  
      | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
  0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">let invalid3: Array&lt;Foo&lt;Bar&gt;&gt;;</span>
    0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">let invalid3: Foo&lt;Bar&gt;[];</span>
  
</code></pre>{% endraw %}

```ts
let invalid: Array<[number, number]>;
```

{% raw %}<pre class="language-text"><code class="language-text">style/useShorthandArrayType.js:1:14 <a href="https://rome.tools/docs/lint/rules/useShorthandArrayType">lint/style/useShorthandArrayType</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Use </span><span style="color: Tomato;"><strong>shorthand T[] syntax</strong></span><span style="color: Tomato;"> instead of </span><span style="color: Tomato;"><strong>Array&lt;T&gt; syntax.</strong></span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> style/useShorthandArrayType.js:1:14
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> let invalid: <span style="color: Tomato;">A</span><span style="color: Tomato;">r</span><span style="color: Tomato;">r</span><span style="color: Tomato;">a</span><span style="color: Tomato;">y</span><span style="color: Tomato;">&lt;</span><span style="color: Tomato;">[</span><span style="color: Tomato;">n</span><span style="color: Tomato;">u</span><span style="color: Tomato;">m</span><span style="color: Tomato;">b</span><span style="color: Tomato;">e</span><span style="color: Tomato;">r</span><span style="color: Tomato;">,</span><span style="color: Tomato;"> </span><span style="color: Tomato;">n</span><span style="color: Tomato;">u</span><span style="color: Tomato;">m</span><span style="color: Tomato;">b</span><span style="color: Tomato;">e</span><span style="color: Tomato;">r</span><span style="color: Tomato;">]</span><span style="color: Tomato;">&gt;</span>;
    <span style="color: rgb(38, 148, 255);">│</span>              <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Use </span><span style="color: rgb(38, 148, 255);"><strong>shorthand T[] syntax</strong></span><span style="color: rgb(38, 148, 255);"> to replace</span>
  
      | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
  0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">let invalid: Array&lt;[number, number]&gt;;</span>
    0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">let invalid: [number, number][];</span>
  
</code></pre>{% endraw %}

```ts
let invalid: Array<[number, number]>;
```

{% raw %}<pre class="language-text"><code class="language-text">style/useShorthandArrayType.js:1:14 <a href="https://rome.tools/docs/lint/rules/useShorthandArrayType">lint/style/useShorthandArrayType</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Use </span><span style="color: Tomato;"><strong>shorthand T[] syntax</strong></span><span style="color: Tomato;"> instead of </span><span style="color: Tomato;"><strong>Array&lt;T&gt; syntax.</strong></span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> style/useShorthandArrayType.js:1:14
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> let invalid: <span style="color: Tomato;">A</span><span style="color: Tomato;">r</span><span style="color: Tomato;">r</span><span style="color: Tomato;">a</span><span style="color: Tomato;">y</span><span style="color: Tomato;">&lt;</span><span style="color: Tomato;">[</span><span style="color: Tomato;">n</span><span style="color: Tomato;">u</span><span style="color: Tomato;">m</span><span style="color: Tomato;">b</span><span style="color: Tomato;">e</span><span style="color: Tomato;">r</span><span style="color: Tomato;">,</span><span style="color: Tomato;"> </span><span style="color: Tomato;">n</span><span style="color: Tomato;">u</span><span style="color: Tomato;">m</span><span style="color: Tomato;">b</span><span style="color: Tomato;">e</span><span style="color: Tomato;">r</span><span style="color: Tomato;">]</span><span style="color: Tomato;">&gt;</span>;
    <span style="color: rgb(38, 148, 255);">│</span>              <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Use </span><span style="color: rgb(38, 148, 255);"><strong>shorthand T[] syntax</strong></span><span style="color: rgb(38, 148, 255);"> to replace</span>
  
      | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
  0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">let invalid: Array&lt;[number, number]&gt;;</span>
    0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">let invalid: [number, number][];</span>
  
</code></pre>{% endraw %}

### Valid

```ts
let valid: Array<Foo | Bar>;
let valid: Array<keyof Bar>;
let valid: Array<foo | bar>;
```

