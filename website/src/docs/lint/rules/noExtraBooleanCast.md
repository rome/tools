---
title: Lint Rule noExtraBooleanCast
layout: layouts/rule.liquid
---

# noExtraBooleanCast (since v0.9.0)

> This rule is recommended by Rome.

Disallow unnecessary boolean casts

## Examples

### Invalid

```jsx
if (!Boolean(foo)) {
}
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noExtraBooleanCast/">js/noExtraBooleanCast</a></span><span style="color: Orange;">]</span><em>: </em><em>Avoid redundant `Boolean` call</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noExtraBooleanCast.js:1:6
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> if (!Boolean(foo)) {
  <span style="color: rgb(38, 148, 255);">│</span>      <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove redundant `Boolean` call</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1,2 +1,2 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">if (!Boolean(foo)) {</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">if (!foo) {</span>
1 1 |   }

=  note: It is not necessary to use `Boolean` call when a value will already be coerced to a boolean.

</code></pre>{% endraw %}

```jsx
while (!!foo) {}
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noExtraBooleanCast/">js/noExtraBooleanCast</a></span><span style="color: Orange;">]</span><em>: </em><em>Avoid redundant double-negation.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noExtraBooleanCast.js:1:8
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> while (!!foo) {}
  <span style="color: rgb(38, 148, 255);">│</span>        <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove redundant double-negation</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">while (!!foo) {}</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">while (foo) {}</span>

=  note: It is not necessary to use double-negation when a value will already be coerced to a boolean.

</code></pre>{% endraw %}

```jsx
let x = 1;
do {
1 + 1;
} while (Boolean(x));
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noExtraBooleanCast/">js/noExtraBooleanCast</a></span><span style="color: Orange;">]</span><em>: </em><em>Avoid redundant `Boolean` call</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noExtraBooleanCast.js:4:10
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">4</span> <span style="color: rgb(38, 148, 255);">│</span> } while (Boolean(x));
  <span style="color: rgb(38, 148, 255);">│</span>          <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove redundant `Boolean` call</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1,4 +1,4 @@</span>
0 0 |   let x = 1;
1 1 |   do {
2 2 |   1 + 1;
3   | <span style="color: Tomato;">- </span><span style="color: Tomato;">} while (Boolean(x));</span>
  3 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">} while (x);</span>

=  note: It is not necessary to use `Boolean` call when a value will already be coerced to a boolean.

</code></pre>{% endraw %}

```jsx
for (; !!foo; ) {}
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noExtraBooleanCast/">js/noExtraBooleanCast</a></span><span style="color: Orange;">]</span><em>: </em><em>Avoid redundant double-negation.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noExtraBooleanCast.js:1:8
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> for (; !!foo; ) {}
  <span style="color: rgb(38, 148, 255);">│</span>        <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove redundant double-negation</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">for (; !!foo; ) {}</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">for (; foo; ) {}</span>

=  note: It is not necessary to use double-negation when a value will already be coerced to a boolean.

</code></pre>{% endraw %}

```jsx
new Boolean(!!x);
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noExtraBooleanCast/">js/noExtraBooleanCast</a></span><span style="color: Orange;">]</span><em>: </em><em>Avoid redundant double-negation.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noExtraBooleanCast.js:1:13
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> new Boolean(!!x);
  <span style="color: rgb(38, 148, 255);">│</span>             <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove redundant double-negation</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">new Boolean(!!x);</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">new Boolean(x);</span>

=  note: It is not necessary to use double-negation when a value will already be coerced to a boolean.

</code></pre>{% endraw %}

### Valid

```jsx
Boolean(!x);
!x;
!!x;
```

