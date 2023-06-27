---
title: Lint Rule noNonoctalDecimalEscape
parent: lint/rules/index
---

# noNonoctalDecimalEscape (since vnext)

Disallow `\8` and `\9` escape sequences in string literals.

Since ECMAScript 2021, the escape sequences \8 and \9 have been defined as non-octal decimal escape sequences.
However, most JavaScript engines consider them to be "useless" escapes. For example:

```jsx
"\8" === "8"; // true
"\9" === "9"; // true
```

nursery/noNonoctalDecimalEscape.js:1:1 <a href="https://docs.rome.tools/lint/rules/noNonoctalDecimalEscape">lint/nursery/noNonoctalDecimalEscape</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Don't use `\8` and `\9` escape sequences in string literals.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&quot;\8&quot; === &quot;8&quot;; // true
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>&quot;\9&quot; === &quot;9&quot;; // true
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Replace </span><span style="color: rgb(38, 148, 255);"><strong>\8</strong></span><span style="color: rgb(38, 148, 255);"> with </span><span style="color: rgb(38, 148, 255);"><strong>\\8</strong></span><span style="color: rgb(38, 148, 255);"> to include the actual backslash character.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;">\</span><span style="color: Tomato;">8</span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">=</span><span style="color: Tomato;">=</span><span style="color: Tomato;">=</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">8</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">;</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">/</span><span style="color: Tomato;">/</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">t</span><span style="color: Tomato;">r</span><span style="color: Tomato;">u</span><span style="color: Tomato;">e</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">\</span><span style="color: MediumSeaGreen;"><strong>\</strong></span><span style="color: MediumSeaGreen;">8</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">&quot;</span><span style="color: MediumSeaGreen;">8</span><span style="color: MediumSeaGreen;">&quot;</span><span style="color: MediumSeaGreen;">;</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">/</span><span style="color: MediumSeaGreen;">/</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">u</span><span style="color: MediumSeaGreen;">e</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  &quot;\9&quot; === &quot;9&quot;; // true
    <strong>3</strong> <strong>3</strong><strong> │ </strong>  
  
Although this syntax is deprecated, it is still supported for compatibility reasons.
If the ECMAScript host is not a web browser, this syntax is optional.
However, web browsers are still required to support it, but only in non-strict mode.
Regardless of your targeted environment, it is recommended to avoid using these escape sequences in new code.

Source: https://eslint.org/docs/latest/rules/no-nonoctal-decimal-escape

## Examples

### Invalid

```jsx
const x = "\8";
```

<pre class="language-text"><code class="language-text">nursery/noNonoctalDecimalEscape.js:1:11 <a href="https://docs.rome.tools/lint/rules/noNonoctalDecimalEscape">lint/nursery/noNonoctalDecimalEscape</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Don't use `\8` and `\9` escape sequences in string literals.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>const x = &quot;\8&quot;;
   <strong>   │ </strong>          <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Replace </span><span style="color: rgb(38, 148, 255);"><strong>\8</strong></span><span style="color: rgb(38, 148, 255);"> with </span><span style="color: rgb(38, 148, 255);"><strong>\\8</strong></span><span style="color: rgb(38, 148, 255);"> to include the actual backslash character.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">c</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;">s</span><span style="color: Tomato;">t</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">x</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">=</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;">\</span><span style="color: Tomato;">8</span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">c</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">x</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">\</span><span style="color: MediumSeaGreen;"><strong>\</strong></span><span style="color: MediumSeaGreen;">8</span><span style="color: MediumSeaGreen;">;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

```jsx
const x = "Don't use \8 and \9 escapes.";
```

<pre class="language-text"><code class="language-text">nursery/noNonoctalDecimalEscape.js:1:21 <a href="https://docs.rome.tools/lint/rules/noNonoctalDecimalEscape">lint/nursery/noNonoctalDecimalEscape</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Don't use `\8` and `\9` escape sequences in string literals.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>const x = &quot;Don't use \8 and \9 escapes.&quot;;
   <strong>   │ </strong>                    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Replace </span><span style="color: rgb(38, 148, 255);"><strong>\8</strong></span><span style="color: rgb(38, 148, 255);"> with </span><span style="color: rgb(38, 148, 255);"><strong>8</strong></span><span style="color: rgb(38, 148, 255);">. This maintains the current functionality.</span>
  
<strong>  </strong><strong>  1 │ </strong>const<span style="opacity: 0.8;">·</span>x<span style="opacity: 0.8;">·</span>=<span style="opacity: 0.8;">·</span><span style="color: Tomato;">&quot;</span>Don't<span style="opacity: 0.8;">·</span>use<span style="opacity: 0.8;">·</span><span style="color: Tomato;">\</span>8<span style="opacity: 0.8;">·</span>and<span style="opacity: 0.8;">·</span>\9<span style="opacity: 0.8;">·</span>escapes.<span style="color: Tomato;">&quot;</span>;
<strong>  </strong><strong>    │ </strong>          <span style="color: Tomato;">-</span>          <span style="color: Tomato;">-</span>                 <span style="color: Tomato;">-</span> 
nursery/noNonoctalDecimalEscape.js:1:21 <a href="https://docs.rome.tools/lint/rules/noNonoctalDecimalEscape">lint/nursery/noNonoctalDecimalEscape</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Don't use `\8` and `\9` escape sequences in string literals.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>const x = &quot;Don't use \8 and \9 escapes.&quot;;
   <strong>   │ </strong>                    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Replace </span><span style="color: rgb(38, 148, 255);"><strong>\8</strong></span><span style="color: rgb(38, 148, 255);"> with </span><span style="color: rgb(38, 148, 255);"><strong>\\8</strong></span><span style="color: rgb(38, 148, 255);"> to include the actual backslash character.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">c</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;">s</span><span style="color: Tomato;">t</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">x</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">=</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;">D</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;">'</span><span style="color: Tomato;">t</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">u</span><span style="color: Tomato;">s</span><span style="color: Tomato;">e</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">\</span><span style="color: Tomato;">8</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">a</span><span style="color: Tomato;">n</span><span style="color: Tomato;">d</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">\</span><span style="color: Tomato;">9</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">e</span><span style="color: Tomato;">s</span><span style="color: Tomato;">c</span><span style="color: Tomato;">a</span><span style="color: Tomato;">p</span><span style="color: Tomato;">e</span><span style="color: Tomato;">s</span><span style="color: Tomato;">.</span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">c</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">x</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">D</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">'</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">u</span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">\</span><span style="color: MediumSeaGreen;"><strong>\</strong></span><span style="color: MediumSeaGreen;">8</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">d</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">\</span><span style="color: MediumSeaGreen;">9</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;">c</span><span style="color: MediumSeaGreen;">a</span><span style="color: MediumSeaGreen;">p</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;">.</span><span style="color: MediumSeaGreen;">;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

```jsx
const x = "\0\8";
```

<pre class="language-text"><code class="language-text">nursery/noNonoctalDecimalEscape.js:1:11 <a href="https://docs.rome.tools/lint/rules/noNonoctalDecimalEscape">lint/nursery/noNonoctalDecimalEscape</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Don't use `\8` and `\9` escape sequences in string literals.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>const x = &quot;\0\8&quot;;
   <strong>   │ </strong>          <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Replace </span><span style="color: rgb(38, 148, 255);"><strong>\0\8</strong></span><span style="color: rgb(38, 148, 255);"> with </span><span style="color: rgb(38, 148, 255);"><strong>\u00008</strong></span><span style="color: rgb(38, 148, 255);">. This maintains the current functionality.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">c</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;">s</span><span style="color: Tomato;">t</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">x</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">=</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;">\</span><span style="color: Tomato;"><strong>0</strong></span><span style="color: Tomato;"><strong>\</strong></span><span style="color: Tomato;"><strong>8</strong></span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">c</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">x</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">\</span><span style="color: MediumSeaGreen;"><strong>u</strong></span><span style="color: MediumSeaGreen;"><strong>0</strong></span><span style="color: MediumSeaGreen;"><strong>0</strong></span><span style="color: MediumSeaGreen;"><strong>0</strong></span><span style="color: MediumSeaGreen;"><strong>0</strong></span><span style="color: MediumSeaGreen;"><strong>8</strong></span><span style="color: MediumSeaGreen;">;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
nursery/noNonoctalDecimalEscape.js:1:13 <a href="https://docs.rome.tools/lint/rules/noNonoctalDecimalEscape">lint/nursery/noNonoctalDecimalEscape</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Don't use `\8` and `\9` escape sequences in string literals.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>const x = &quot;\0\8&quot;;
   <strong>   │ </strong>            <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Replace </span><span style="color: rgb(38, 148, 255);"><strong>\8</strong></span><span style="color: rgb(38, 148, 255);"> with </span><span style="color: rgb(38, 148, 255);"><strong>\u0038</strong></span><span style="color: rgb(38, 148, 255);">. This maintains the current functionality.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">c</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;">s</span><span style="color: Tomato;">t</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">x</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">=</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;">\</span><span style="color: Tomato;">0</span><span style="color: Tomato;">\</span><span style="color: Tomato;"><strong>8</strong></span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">c</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">x</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">\</span><span style="color: MediumSeaGreen;">0</span><span style="color: MediumSeaGreen;">\</span><span style="color: MediumSeaGreen;"><strong>u</strong></span><span style="color: MediumSeaGreen;"><strong>0</strong></span><span style="color: MediumSeaGreen;"><strong>0</strong></span><span style="color: MediumSeaGreen;"><strong>3</strong></span><span style="color: MediumSeaGreen;"><strong>8</strong></span><span style="color: MediumSeaGreen;">;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

## Valid

```jsx
const x = "8";
```

```jsx
const x = "Don't use \\8 and \\9 escapes.";
```

```jsx
const x = "\0\u0038";;
```

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
