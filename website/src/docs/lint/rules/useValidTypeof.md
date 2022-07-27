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

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/useValidTypeof/">js/useValidTypeof</a></span><span style="color: Orange;">]</span><em>: </em><em>Invalid `typeof` comparison value</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/useValidTypeof.js:1:16
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> typeof foo === &quot;strnig&quot;
  <span style="color: rgb(38, 148, 255);">│</span>                <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">not a valid type name</span>

</code></pre>{% endraw %}

```jsx
typeof foo == "undefimed"
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/useValidTypeof/">js/useValidTypeof</a></span><span style="color: Orange;">]</span><em>: </em><em>Invalid `typeof` comparison value</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/useValidTypeof.js:1:15
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> typeof foo == &quot;undefimed&quot;
  <span style="color: rgb(38, 148, 255);">│</span>               <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">not a valid type name</span>

</code></pre>{% endraw %}

```jsx
typeof bar != "nunber"
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/useValidTypeof/">js/useValidTypeof</a></span><span style="color: Orange;">]</span><em>: </em><em>Invalid `typeof` comparison value</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/useValidTypeof.js:1:15
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> typeof bar != &quot;nunber&quot;
  <span style="color: rgb(38, 148, 255);">│</span>               <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">not a valid type name</span>

</code></pre>{% endraw %}

```jsx
typeof bar !== "fucntion"
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/useValidTypeof/">js/useValidTypeof</a></span><span style="color: Orange;">]</span><em>: </em><em>Invalid `typeof` comparison value</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/useValidTypeof.js:1:16
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> typeof bar !== &quot;fucntion&quot;
  <span style="color: rgb(38, 148, 255);">│</span>                <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">not a valid type name</span>

</code></pre>{% endraw %}

```jsx
typeof foo === undefined
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/useValidTypeof/">js/useValidTypeof</a></span><span style="color: Orange;">]</span><em>: </em><em>Invalid `typeof` comparison value</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/useValidTypeof.js:1:16
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> typeof foo === undefined
  <span style="color: rgb(38, 148, 255);">│</span>                <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">not a string literal</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Compare the result of `typeof` with a valid type name</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">typeof foo === undefined</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">typeof foo === &quot;undefined&quot;</span>

</code></pre>{% endraw %}

```jsx
typeof bar == Object
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/useValidTypeof/">js/useValidTypeof</a></span><span style="color: Orange;">]</span><em>: </em><em>Invalid `typeof` comparison value</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/useValidTypeof.js:1:15
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> typeof bar == Object
  <span style="color: rgb(38, 148, 255);">│</span>               <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">not a string literal</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Compare the result of `typeof` with a valid type name</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">typeof bar == Object</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">typeof bar == &quot;object&quot;</span>

</code></pre>{% endraw %}

```jsx
typeof foo === baz
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/useValidTypeof/">js/useValidTypeof</a></span><span style="color: Orange;">]</span><em>: </em><em>Invalid `typeof` comparison value</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/useValidTypeof.js:1:16
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> typeof foo === baz
  <span style="color: rgb(38, 148, 255);">│</span>                <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">not a string literal</span>

</code></pre>{% endraw %}

```jsx
typeof foo == 5
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/useValidTypeof/">js/useValidTypeof</a></span><span style="color: Orange;">]</span><em>: </em><em>Invalid `typeof` comparison value</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/useValidTypeof.js:1:15
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> typeof foo == 5
  <span style="color: rgb(38, 148, 255);">│</span>               <span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">not a string literal</span>

</code></pre>{% endraw %}

```jsx
typeof foo == -5
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/useValidTypeof/">js/useValidTypeof</a></span><span style="color: Orange;">]</span><em>: </em><em>Invalid `typeof` comparison value</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/useValidTypeof.js:1:15
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> typeof foo == -5
  <span style="color: rgb(38, 148, 255);">│</span>               <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">not a string literal</span>

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

