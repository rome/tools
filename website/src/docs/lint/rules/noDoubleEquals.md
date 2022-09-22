---
title: Lint Rule noDoubleEquals
layout: layouts/rule.liquid
---

# noDoubleEquals (since v0.7.0)

> This rule is recommended by Rome.

Require the use of `===` and `!==`

It is generally bad practice to use `==` for comparison instead of
`===`. Double operators will triger implicit [type coercion](https://developer.mozilla.org/en-US/docs/Glossary/Type_coercion)
and are thus not prefered. Using strict equality operators is almost
always best practice.

For ergonomic reasons, this rule makes an exception for `== null` for
comparing to both `null` and `undefined`.

## Examples

### Invalid

```jsx
foo == bar
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noDoubleEquals/">correctness/noDoubleEquals</a></span><span style="color: Tomato;">]</span><strong>: </strong><strong>Use </strong><strong><strong>===</strong></strong><strong> instead of </strong><strong><strong>==</strong></strong>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noDoubleEquals.js:1:5
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> foo <span style="color: Tomato;">=</span><span style="color: Tomato;">=</span> bar
  <span style="color: rgb(38, 148, 255);">│</span>     <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span> <span style="color: Tomato;"><strong>==</strong></span><span style="color: Tomato;"> is only allowed when comparing against </span><span style="color: Tomato;"><strong>null</strong></span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Use </span><span style="color: rgb(38, 148, 255);"><strong>===</strong></span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">foo == bar</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">foo === bar</span>

=  note: Using <strong>===</strong> may be unsafe if you are relying on type coercion

</code></pre>{% endraw %}

### Valid

```jsx
foo == null
```

```jsx
foo != null
```

```jsx
null == foo
```

```jsx
null != foo
```

