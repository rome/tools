---
title: Lint Rule noExtraBooleanCast
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

{% raw %}<pre class="language-text"><code class="language-text">complexity/noExtraBooleanCast.js:1:6 <a href="https://docs.rome.tools/lint/rules/noExtraBooleanCast">lint/complexity/noExtraBooleanCast</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid redundant `Boolean` call</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>if (!Boolean(foo)) {
   <strong>   │ </strong>     <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>}
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">It is not necessary to use `Boolean` call when a value will already be coerced to a boolean.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove redundant `Boolean` call</span>
  
<strong>  </strong><strong>  1 │ </strong>if<span style="opacity: 0.8;">·</span>(!<span style="color: Tomato;">B</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span><span style="color: Tomato;">l</span><span style="color: Tomato;">e</span><span style="color: Tomato;">a</span><span style="color: Tomato;">n</span><span style="color: Tomato;">(</span>foo<span style="color: Tomato;">)</span>)<span style="opacity: 0.8;">·</span>{
<strong>  </strong><strong>    │ </strong>     <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>   <span style="color: Tomato;">-</span>   
</code></pre>{% endraw %}

```jsx
while (!!foo) {}
```

{% raw %}<pre class="language-text"><code class="language-text">complexity/noExtraBooleanCast.js:1:8 <a href="https://docs.rome.tools/lint/rules/noExtraBooleanCast">lint/complexity/noExtraBooleanCast</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid redundant double-negation.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>while (!!foo) {}
   <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">It is not necessary to use double-negation when a value will already be coerced to a boolean.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove redundant double-negation</span>
  
<strong>  </strong><strong>  1 │ </strong>while<span style="opacity: 0.8;">·</span>(<span style="color: Tomato;">!</span><span style="color: Tomato;">!</span>foo)<span style="opacity: 0.8;">·</span>{}
<strong>  </strong><strong>    │ </strong>       <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>       
</code></pre>{% endraw %}

```jsx
let x = 1;
do {
1 + 1;
} while (Boolean(x));
```

{% raw %}<pre class="language-text"><code class="language-text">complexity/noExtraBooleanCast.js:4:10 <a href="https://docs.rome.tools/lint/rules/noExtraBooleanCast">lint/complexity/noExtraBooleanCast</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid redundant `Boolean` call</span>
  
    <strong>2 │ </strong>do {
    <strong>3 │ </strong>1 + 1;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>} while (Boolean(x));
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">It is not necessary to use `Boolean` call when a value will already be coerced to a boolean.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove redundant `Boolean` call</span>
  
<strong>  </strong><strong>  4 │ </strong>}<span style="opacity: 0.8;">·</span>while<span style="opacity: 0.8;">·</span>(<span style="color: Tomato;">B</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span><span style="color: Tomato;">l</span><span style="color: Tomato;">e</span><span style="color: Tomato;">a</span><span style="color: Tomato;">n</span><span style="color: Tomato;">(</span>x<span style="color: Tomato;">)</span>);
<strong>  </strong><strong>    │ </strong>         <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span> <span style="color: Tomato;">-</span>  
</code></pre>{% endraw %}

```jsx
for (; !!foo; ) {}
```

{% raw %}<pre class="language-text"><code class="language-text">complexity/noExtraBooleanCast.js:1:8 <a href="https://docs.rome.tools/lint/rules/noExtraBooleanCast">lint/complexity/noExtraBooleanCast</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid redundant double-negation.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>for (; !!foo; ) {}
   <strong>   │ </strong>       <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">It is not necessary to use double-negation when a value will already be coerced to a boolean.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove redundant double-negation</span>
  
<strong>  </strong><strong>  1 │ </strong>for<span style="opacity: 0.8;">·</span>(;<span style="opacity: 0.8;">·</span><span style="color: Tomato;">!</span><span style="color: Tomato;">!</span>foo;<span style="opacity: 0.8;">·</span>)<span style="opacity: 0.8;">·</span>{}
<strong>  </strong><strong>    │ </strong>       <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>         
</code></pre>{% endraw %}

```jsx
new Boolean(!!x);
```

{% raw %}<pre class="language-text"><code class="language-text">complexity/noExtraBooleanCast.js:1:13 <a href="https://docs.rome.tools/lint/rules/noExtraBooleanCast">lint/complexity/noExtraBooleanCast</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid redundant double-negation.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>new Boolean(!!x);
   <strong>   │ </strong>            <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">It is not necessary to use double-negation when a value will already be coerced to a boolean.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove redundant double-negation</span>
  
<strong>  </strong><strong>  1 │ </strong>new<span style="opacity: 0.8;">·</span>Boolean(<span style="color: Tomato;">!</span><span style="color: Tomato;">!</span>x);
<strong>  </strong><strong>    │ </strong>            <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>   
</code></pre>{% endraw %}

### Valid

```jsx
Boolean(!x);
!x;
!!x;
```

