---
title: Lint Rule useLiteralKeys
parent: lint/rules/index
---

# useLiteralKeys (since v12.1.0)

Enforce the usage of a literal access to properties over computed property access.

## Examples

### Invalid

```jsx
a.b["c"];
```

<pre class="language-text"><code class="language-text">nursery/useLiteralKeys.js:1:5 <a href="https://docs.rome.tools/lint/rules/useLiteralKeys">lint/nursery/useLiteralKeys</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The computed expression can be simplified without the use of a string literal.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>a.b[&quot;c&quot;];
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Replace it with a static expression.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>.</strong></span><span style="color: Tomato;"><strong>b</strong></span><span style="color: Tomato;"><strong>[</strong></span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;"><strong>c</strong></span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;"><strong>]</strong></span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><strong>.</strong></span><span style="color: MediumSeaGreen;"><strong>b</strong></span><span style="color: MediumSeaGreen;"><strong>.</strong></span><span style="color: MediumSeaGreen;"><strong>c</strong></span><span style="color: MediumSeaGreen;">;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

```jsx
a.c[`d`]
```

<pre class="language-text"><code class="language-text">nursery/useLiteralKeys.js:1:5 <a href="https://docs.rome.tools/lint/rules/useLiteralKeys">lint/nursery/useLiteralKeys</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The computed expression can be simplified without the use of a string literal.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>a.c[`d`]
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Replace it with a static expression.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>.</strong></span><span style="color: Tomato;"><strong>c</strong></span><span style="color: Tomato;"><strong>[</strong></span><span style="color: Tomato;"><strong>`</strong></span><span style="color: Tomato;"><strong>d</strong></span><span style="color: Tomato;"><strong>`</strong></span><span style="color: Tomato;"><strong>]</strong></span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><strong>.</strong></span><span style="color: MediumSeaGreen;"><strong>c</strong></span><span style="color: MediumSeaGreen;"><strong>.</strong></span><span style="color: MediumSeaGreen;"><strong>d</strong></span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

```jsx
a.c[`d`] = "something"
```

<pre class="language-text"><code class="language-text">nursery/useLiteralKeys.js:1:5 <a href="https://docs.rome.tools/lint/rules/useLiteralKeys">lint/nursery/useLiteralKeys</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The computed expression can be simplified without the use of a string literal.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>a.c[`d`] = &quot;something&quot;
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Replace it with a static expression.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>.</strong></span><span style="color: Tomato;"><strong>c</strong></span><span style="color: Tomato;"><strong>[</strong></span><span style="color: Tomato;"><strong>`</strong></span><span style="color: Tomato;"><strong>d</strong></span><span style="color: Tomato;"><strong>`</strong></span><span style="color: Tomato;"><strong>]</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">=</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">s</span><span style="color: Tomato;">o</span><span style="color: Tomato;">m</span><span style="color: Tomato;">e</span><span style="color: Tomato;">t</span><span style="color: Tomato;">h</span><span style="color: Tomato;">i</span><span style="color: Tomato;">n</span><span style="color: Tomato;">g</span><span style="color: Tomato;">&quot;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>a</strong></span><span style="color: MediumSeaGreen;"><strong>.</strong></span><span style="color: MediumSeaGreen;"><strong>c</strong></span><span style="color: MediumSeaGreen;"><strong>.</strong></span><span style="color: MediumSeaGreen;"><strong>d</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">&quot;</span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">m</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">h</span><span style="color: MediumSeaGreen;">i</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">g</span><span style="color: MediumSeaGreen;">&quot;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

```jsx
a = {
	['b']: d
}
```

<pre class="language-text"><code class="language-text">nursery/useLiteralKeys.js:2:3 <a href="https://docs.rome.tools/lint/rules/useLiteralKeys">lint/nursery/useLiteralKeys</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The computed expression can be simplified without the use of a string literal.</span>
  
    <strong>1 │ </strong>a = {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>	['b']: d
   <strong>   │ </strong>	 <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>}
    <strong>4 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Replace it with a static expression.</span>
  
<strong>  </strong><strong>  2 │ </strong><span style="opacity: 0.8;">→ </span><span style="color: Tomato;">[</span><span style="color: Tomato;">'</span>b<span style="color: Tomato;">'</span><span style="color: Tomato;">]</span>:<span style="opacity: 0.8;">·</span>d
<strong>  </strong><strong>    │ </strong>  <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span> <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>   
</code></pre>

## Valid

```jsx
a["c" + "d"];
a[d.c];
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
