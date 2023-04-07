---
title: Lint Rule useLiteralKeys
parent: lint/rules/index
---

# useLiteralKeys (since vnext)

Enforce the usage of a computed expression over a static expression with strings.

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

## Valid

```jsx
a["c" + "d"];
a[d.c];
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
