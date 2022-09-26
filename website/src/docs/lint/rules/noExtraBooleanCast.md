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

{% raw %}<pre class="language-text"><code class="language-text">correctness/noExtraBooleanCast.js:1:6 <a href="https://rome.tools/docs/lint/rules/noExtraBooleanCast">lint/correctness/noExtraBooleanCast</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid redundant `Boolean` call</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>if (!Boolean(foo)) {
   <strong>   │ </strong>     <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>}
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">It is not necessary to use `Boolean` call when a value will already be coerced to a boolean.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove redundant `Boolean` call</span>
  
      | <span style="color: rgb(38, 148, 255);">@@ -1,2 +1,2 @@</span>
  0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">if (!Boolean(foo)) {</span>
    0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">if (!foo) {</span>
  1 1 |   }
  
</code></pre>{% endraw %}

```jsx
while (!!foo) {}
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noExtraBooleanCast.js:1:8 <a href="https://rome.tools/docs/lint/rules/noExtraBooleanCast">lint/correctness/noExtraBooleanCast</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid redundant double-negation.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>while (!!foo) {}
   <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">It is not necessary to use double-negation when a value will already be coerced to a boolean.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove redundant double-negation</span>
  
      | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
  0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">while (!!foo) {}</span>
    0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">while (foo) {}</span>
  
</code></pre>{% endraw %}

```jsx
let x = 1;
do {
1 + 1;
} while (Boolean(x));
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noExtraBooleanCast.js:4:10 <a href="https://rome.tools/docs/lint/rules/noExtraBooleanCast">lint/correctness/noExtraBooleanCast</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid redundant `Boolean` call</span>
  
    <strong>2 │ </strong>do {
    <strong>3 │ </strong>1 + 1;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>} while (Boolean(x));
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">It is not necessary to use `Boolean` call when a value will already be coerced to a boolean.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove redundant `Boolean` call</span>
  
      | <span style="color: rgb(38, 148, 255);">@@ -1,4 +1,4 @@</span>
  0 0 |   let x = 1;
  1 1 |   do {
  2 2 |   1 + 1;
  3   | <span style="color: Tomato;">- </span><span style="color: Tomato;">} while (Boolean(x));</span>
    3 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">} while (x);</span>
  
</code></pre>{% endraw %}

```jsx
for (; !!foo; ) {}
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noExtraBooleanCast.js:1:8 <a href="https://rome.tools/docs/lint/rules/noExtraBooleanCast">lint/correctness/noExtraBooleanCast</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid redundant double-negation.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>for (; !!foo; ) {}
   <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">It is not necessary to use double-negation when a value will already be coerced to a boolean.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove redundant double-negation</span>
  
      | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
  0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">for (; !!foo; ) {}</span>
    0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">for (; foo; ) {}</span>
  
</code></pre>{% endraw %}

```jsx
new Boolean(!!x);
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noExtraBooleanCast.js:1:13 <a href="https://rome.tools/docs/lint/rules/noExtraBooleanCast">lint/correctness/noExtraBooleanCast</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid redundant double-negation.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>new Boolean(!!x);
   <strong>   │ </strong>            <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">It is not necessary to use double-negation when a value will already be coerced to a boolean.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove redundant double-negation</span>
  
      | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
  0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">new Boolean(!!x);</span>
    0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">new Boolean(x);</span>
  
</code></pre>{% endraw %}

### Valid

```jsx
Boolean(!x);
!x;
!!x;
```

