---
title: Lint Rule useValidTypeof
layout: layouts/rule.liquid
---

# useValidTypeof (since v0.7.0)

> This rule is recommended by Rome.

This rule verifies the result of `typeof $expr` unary expressions is being
compared to valid values, either string literals containing valid type
names or other `typeof` expressions

## Examples

### Invalid

```jsx
typeof foo === "strnig"
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/useValidTypeof.js:1:16 <a href="https://rome.tools/docs/lint/rules/useValidTypeof">lint/correctness/useValidTypeof</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Invalid `typeof` comparison value</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">not a valid type name</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/useValidTypeof.js:1:16
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> typeof foo === <span style="color: Tomato;">&quot;</span><span style="color: Tomato;">s</span><span style="color: Tomato;">t</span><span style="color: Tomato;">r</span><span style="color: Tomato;">n</span><span style="color: Tomato;">i</span><span style="color: Tomato;">g</span><span style="color: Tomato;">&quot;</span>
    <span style="color: rgb(38, 148, 255);">│</span>                <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
</code></pre>{% endraw %}

```jsx
typeof foo == "undefimed"
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/useValidTypeof.js:1:15 <a href="https://rome.tools/docs/lint/rules/useValidTypeof">lint/correctness/useValidTypeof</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Invalid `typeof` comparison value</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">not a valid type name</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/useValidTypeof.js:1:15
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> typeof foo == <span style="color: Tomato;">&quot;</span><span style="color: Tomato;">u</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;">e</span><span style="color: Tomato;">f</span><span style="color: Tomato;">i</span><span style="color: Tomato;">m</span><span style="color: Tomato;">e</span><span style="color: Tomato;">d</span><span style="color: Tomato;">&quot;</span>
    <span style="color: rgb(38, 148, 255);">│</span>               <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
</code></pre>{% endraw %}

```jsx
typeof bar != "nunber"
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/useValidTypeof.js:1:15 <a href="https://rome.tools/docs/lint/rules/useValidTypeof">lint/correctness/useValidTypeof</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Invalid `typeof` comparison value</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">not a valid type name</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/useValidTypeof.js:1:15
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> typeof bar != <span style="color: Tomato;">&quot;</span><span style="color: Tomato;">n</span><span style="color: Tomato;">u</span><span style="color: Tomato;">n</span><span style="color: Tomato;">b</span><span style="color: Tomato;">e</span><span style="color: Tomato;">r</span><span style="color: Tomato;">&quot;</span>
    <span style="color: rgb(38, 148, 255);">│</span>               <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
</code></pre>{% endraw %}

```jsx
typeof bar !== "fucntion"
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/useValidTypeof.js:1:16 <a href="https://rome.tools/docs/lint/rules/useValidTypeof">lint/correctness/useValidTypeof</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Invalid `typeof` comparison value</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">not a valid type name</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/useValidTypeof.js:1:16
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> typeof bar !== <span style="color: Tomato;">&quot;</span><span style="color: Tomato;">f</span><span style="color: Tomato;">u</span><span style="color: Tomato;">c</span><span style="color: Tomato;">n</span><span style="color: Tomato;">t</span><span style="color: Tomato;">i</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;">&quot;</span>
    <span style="color: rgb(38, 148, 255);">│</span>                <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
</code></pre>{% endraw %}

```jsx
typeof foo === undefined
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/useValidTypeof.js:1:16 <a href="https://rome.tools/docs/lint/rules/useValidTypeof">lint/correctness/useValidTypeof</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Invalid `typeof` comparison value</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">not a string literal</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/useValidTypeof.js:1:16
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> typeof foo === <span style="color: Tomato;">u</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;">e</span><span style="color: Tomato;">f</span><span style="color: Tomato;">i</span><span style="color: Tomato;">n</span><span style="color: Tomato;">e</span><span style="color: Tomato;">d</span>
    <span style="color: rgb(38, 148, 255);">│</span>                <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Compare the result of `typeof` with a valid type name</span>
  
      | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
  0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">typeof foo === undefined</span>
    0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">typeof foo === &quot;undefined&quot;</span>
  
</code></pre>{% endraw %}

```jsx
typeof bar == Object
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/useValidTypeof.js:1:15 <a href="https://rome.tools/docs/lint/rules/useValidTypeof">lint/correctness/useValidTypeof</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Invalid `typeof` comparison value</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">not a string literal</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/useValidTypeof.js:1:15
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> typeof bar == <span style="color: Tomato;">O</span><span style="color: Tomato;">b</span><span style="color: Tomato;">j</span><span style="color: Tomato;">e</span><span style="color: Tomato;">c</span><span style="color: Tomato;">t</span>
    <span style="color: rgb(38, 148, 255);">│</span>               <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Compare the result of `typeof` with a valid type name</span>
  
      | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
  0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">typeof bar == Object</span>
    0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">typeof bar == &quot;object&quot;</span>
  
</code></pre>{% endraw %}

```jsx
typeof foo === baz
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/useValidTypeof.js:1:16 <a href="https://rome.tools/docs/lint/rules/useValidTypeof">lint/correctness/useValidTypeof</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Invalid `typeof` comparison value</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">not a string literal</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/useValidTypeof.js:1:16
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> typeof foo === <span style="color: Tomato;">b</span><span style="color: Tomato;">a</span><span style="color: Tomato;">z</span>
    <span style="color: rgb(38, 148, 255);">│</span>                <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
</code></pre>{% endraw %}

```jsx
typeof foo == 5
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/useValidTypeof.js:1:15 <a href="https://rome.tools/docs/lint/rules/useValidTypeof">lint/correctness/useValidTypeof</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Invalid `typeof` comparison value</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">not a string literal</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/useValidTypeof.js:1:15
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> typeof foo == <span style="color: Tomato;">5</span>
    <span style="color: rgb(38, 148, 255);">│</span>               <span style="color: Tomato;">^</span>
  
</code></pre>{% endraw %}

```jsx
typeof foo == -5
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/useValidTypeof.js:1:15 <a href="https://rome.tools/docs/lint/rules/useValidTypeof">lint/correctness/useValidTypeof</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Invalid `typeof` comparison value</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">not a string literal</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/useValidTypeof.js:1:15
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> typeof foo == <span style="color: Tomato;">-</span><span style="color: Tomato;">5</span>
    <span style="color: rgb(38, 148, 255);">│</span>               <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
</code></pre>{% endraw %}

### Valid

```jsx
typeof foo === "string"
```

```jsx
typeof bar == "undefined"
```

```jsx
typeof bar === typeof qux
```

