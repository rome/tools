---
title: Lint Rule useSimpleNumberKeys
parent: lint/rules/index
---

# useSimpleNumberKeys (since v12.1.0)

Disallow number literal object member names which are not base10 or uses underscore as separator

## Examples

### Invalid

```jsx
({ 0x1: 1 });
```

<pre class="language-text"><code class="language-text">complexity/useSimpleNumberKeys.js:1:4 <a href="https://docs.rome.tools/lint/rules/useSimpleNumberKeys">lint/complexity/useSimpleNumberKeys</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Hexadecimal number literal is not allowed here.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>({ 0x1: 1 });
   <strong>   │ </strong>   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Safe fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Replace 0x1 with 1</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">(</span><span style="color: Tomato;">{</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>0</strong></span><span style="color: Tomato;"><strong>x</strong></span><span style="color: Tomato;"><strong>1</strong></span><span style="color: Tomato;">:</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">1</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">}</span><span style="color: Tomato;">)</span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;">{</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><strong>1</strong></span><span style="color: MediumSeaGreen;">:</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">1</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">}</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;">;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

```jsx
({ 11_1.11: "ee" });
```

<pre class="language-text"><code class="language-text">complexity/useSimpleNumberKeys.js:1:4 <a href="https://docs.rome.tools/lint/rules/useSimpleNumberKeys">lint/complexity/useSimpleNumberKeys</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Number literal with underscore is not allowed here.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>({ 11_1.11: &quot;ee&quot; });
   <strong>   │ </strong>   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Safe fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Replace 11_1.11 with 111.11</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">(</span><span style="color: Tomato;">{</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>1</strong></span><span style="color: Tomato;"><strong>1</strong></span><span style="color: Tomato;"><strong>_</strong></span><span style="color: Tomato;"><strong>1</strong></span><span style="color: Tomato;"><strong>.</strong></span><span style="color: Tomato;"><strong>1</strong></span><span style="color: Tomato;"><strong>1</strong></span><span style="color: Tomato;">:</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">e</span><span style="color: Tomato;">e</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">}</span><span style="color: Tomato;">)</span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;">{</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><strong>1</strong></span><span style="color: MediumSeaGreen;"><strong>1</strong></span><span style="color: MediumSeaGreen;"><strong>1</strong></span><span style="color: MediumSeaGreen;"><strong>.</strong></span><span style="color: MediumSeaGreen;"><strong>1</strong></span><span style="color: MediumSeaGreen;"><strong>1</strong></span><span style="color: MediumSeaGreen;">:</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">&quot;</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">&quot;</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">}</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;">;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

```jsx
({ 0o1: 1 });
```

<pre class="language-text"><code class="language-text">complexity/useSimpleNumberKeys.js:1:4 <a href="https://docs.rome.tools/lint/rules/useSimpleNumberKeys">lint/complexity/useSimpleNumberKeys</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Octal number literal is not allowed here.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>({ 0o1: 1 });
   <strong>   │ </strong>   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Safe fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Replace 0o1 with 1</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">(</span><span style="color: Tomato;">{</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>0</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><strong>1</strong></span><span style="color: Tomato;">:</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">1</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">}</span><span style="color: Tomato;">)</span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;">{</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><strong>1</strong></span><span style="color: MediumSeaGreen;">:</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">1</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">}</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;">;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

```jsx
({ 1n: 1 });
```

<pre class="language-text"><code class="language-text">complexity/useSimpleNumberKeys.js:1:4 <a href="https://docs.rome.tools/lint/rules/useSimpleNumberKeys">lint/complexity/useSimpleNumberKeys</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Bigint is not allowed here.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>({ 1n: 1 });
   <strong>   │ </strong>   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Safe fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Replace 1n with 1</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">(</span><span style="color: Tomato;">{</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>1</strong></span><span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;">:</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">1</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">}</span><span style="color: Tomato;">)</span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;">{</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><strong>1</strong></span><span style="color: MediumSeaGreen;">:</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">1</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">}</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;">;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

```jsx
({ 11_1.11: "ee" });
```

<pre class="language-text"><code class="language-text">complexity/useSimpleNumberKeys.js:1:4 <a href="https://docs.rome.tools/lint/rules/useSimpleNumberKeys">lint/complexity/useSimpleNumberKeys</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Number literal with underscore is not allowed here.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>({ 11_1.11: &quot;ee&quot; });
   <strong>   │ </strong>   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Safe fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Replace 11_1.11 with 111.11</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">(</span><span style="color: Tomato;">{</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>1</strong></span><span style="color: Tomato;"><strong>1</strong></span><span style="color: Tomato;"><strong>_</strong></span><span style="color: Tomato;"><strong>1</strong></span><span style="color: Tomato;"><strong>.</strong></span><span style="color: Tomato;"><strong>1</strong></span><span style="color: Tomato;"><strong>1</strong></span><span style="color: Tomato;">:</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">e</span><span style="color: Tomato;">e</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">}</span><span style="color: Tomato;">)</span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;">{</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><strong>1</strong></span><span style="color: MediumSeaGreen;"><strong>1</strong></span><span style="color: MediumSeaGreen;"><strong>1</strong></span><span style="color: MediumSeaGreen;"><strong>.</strong></span><span style="color: MediumSeaGreen;"><strong>1</strong></span><span style="color: MediumSeaGreen;"><strong>1</strong></span><span style="color: MediumSeaGreen;">:</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">&quot;</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">&quot;</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">}</span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;">;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

## Valid

```jsx
({ 0: "zero" });
({ 122: "integer" });
({ 1.22: "floating point" });
({ 3.1e12: "floating point with e" });
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
