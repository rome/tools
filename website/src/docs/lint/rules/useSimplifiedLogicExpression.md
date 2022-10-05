---
title: Lint Rule useSimplifiedLogicExpression
layout: layouts/rule.liquid
---

# useSimplifiedLogicExpression (since v0.7.0)

> This rule is recommended by Rome.

Discard redundant terms from logical expressions.

## Examples

### Invalid

```jsx
const boolExp = true;
const r = true && boolExp;
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/useSimplifiedLogicExpression.js:2:11 <a href="https://rome.tools/docs/lint/rules/useSimplifiedLogicExpression">lint/correctness/useSimplifiedLogicExpression</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> 

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Logical expression contains unnecessary complexity.</span>
  
    <strong>1 │ </strong>const boolExp = true;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>const r = true &amp;&amp; boolExp;
   <strong>   │ </strong>          <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Discard redundant terms from the logical expression.</span>
  
<strong>  </strong><strong>  2 │ </strong>const<span style="opacity: 0.8;">·</span>r<span style="opacity: 0.8;">·</span>=<span style="opacity: 0.8;">·</span><span style="color: Tomato;">t</span><span style="color: Tomato;">r</span><span style="color: Tomato;">u</span><span style="color: Tomato;">e</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">&amp;</span><span style="color: Tomato;">&amp;</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>boolExp;
<strong>  </strong><strong>    │ </strong>          <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>        
</code></pre>{% endraw %}

```jsx
const boolExp2 = true;
const r2 = boolExp || true;
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/useSimplifiedLogicExpression.js:2:12 <a href="https://rome.tools/docs/lint/rules/useSimplifiedLogicExpression">lint/correctness/useSimplifiedLogicExpression</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> 

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Logical expression contains unnecessary complexity.</span>
  
    <strong>1 │ </strong>const boolExp2 = true;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>const r2 = boolExp || true;
   <strong>   │ </strong>           <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Discard redundant terms from the logical expression.</span>
  
<strong>  </strong><strong>  2 │ </strong>const<span style="opacity: 0.8;">·</span>r2<span style="opacity: 0.8;">·</span>=<span style="opacity: 0.8;">·</span><span style="color: Tomato;">b</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span><span style="color: Tomato;">l</span><span style="color: Tomato;">E</span><span style="color: Tomato;">x</span><span style="color: Tomato;">p</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">|</span><span style="color: Tomato;">|</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>true;
<strong>  </strong><strong>    │ </strong>           <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>     
</code></pre>{% endraw %}

```jsx
const nonNullExp = 123;
const r3 = null ?? nonNullExp;
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/useSimplifiedLogicExpression.js:2:12 <a href="https://rome.tools/docs/lint/rules/useSimplifiedLogicExpression">lint/correctness/useSimplifiedLogicExpression</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> 

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Logical expression contains unnecessary complexity.</span>
  
    <strong>1 │ </strong>const nonNullExp = 123;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>const r3 = null ?? nonNullExp;
   <strong>   │ </strong>           <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Discard redundant terms from the logical expression.</span>
  
<strong>  </strong><strong>  2 │ </strong>const<span style="opacity: 0.8;">·</span>r3<span style="opacity: 0.8;">·</span>=<span style="opacity: 0.8;">·</span><span style="color: Tomato;">n</span><span style="color: Tomato;">u</span><span style="color: Tomato;">l</span><span style="color: Tomato;">l</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span><span style="color: Tomato;">?</span><span style="color: Tomato;">?</span><span style="opacity: 0.8;"><span style="color: Tomato;">·</span></span>nonNullExp;
<strong>  </strong><strong>    │ </strong>           <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>           
</code></pre>{% endraw %}

```jsx
const boolExpr1 = true;
const boolExpr2 = false;
const r4 = !boolExpr1 || !boolExpr2;
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/useSimplifiedLogicExpression.js:3:12 <a href="https://rome.tools/docs/lint/rules/useSimplifiedLogicExpression">lint/correctness/useSimplifiedLogicExpression</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> 

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Logical expression contains unnecessary complexity.</span>
  
    <strong>1 │ </strong>const boolExpr1 = true;
    <strong>2 │ </strong>const boolExpr2 = false;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>const r4 = !boolExpr1 || !boolExpr2;
   <strong>   │ </strong>           <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Reduce the complexity of the logical expression.</span>
  
    <strong>1</strong> <strong>1</strong><strong> │ </strong>  const boolExpr1 = true;
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  const boolExpr2 = false;
    <strong>3</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">c</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;">s</span><span style="color: Tomato;">t</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">r</span><span style="color: Tomato;">4</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">=</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">!</span><span style="color: Tomato;">b</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span><span style="color: Tomato;">l</span><span style="color: Tomato;">E</span><span style="color: Tomato;">x</span><span style="color: Tomato;">p</span><span style="color: Tomato;">r</span><span style="color: Tomato;">1</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>|</strong></span><span style="color: Tomato;"><strong>|</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;"><strong>!</strong></span><span style="color: Tomato;">b</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span><span style="color: Tomato;">l</span><span style="color: Tomato;">E</span><span style="color: Tomato;">x</span><span style="color: Tomato;">p</span><span style="color: Tomato;">r</span><span style="color: Tomato;">2</span><span style="color: Tomato;">;</span>
      <strong>3</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">c</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;">t</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">4</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">!</span><span style="color: MediumSeaGreen;"><strong>(</strong></span><span style="color: MediumSeaGreen;">b</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">l</span><span style="color: MediumSeaGreen;">E</span><span style="color: MediumSeaGreen;">x</span><span style="color: MediumSeaGreen;">p</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">1</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;"><strong>&amp;</strong></span><span style="color: MediumSeaGreen;"><strong>&amp;</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">b</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">l</span><span style="color: MediumSeaGreen;">E</span><span style="color: MediumSeaGreen;">x</span><span style="color: MediumSeaGreen;">p</span><span style="color: MediumSeaGreen;">r</span><span style="color: MediumSeaGreen;">2</span><span style="color: MediumSeaGreen;"><strong>)</strong></span><span style="color: MediumSeaGreen;">;</span>
    <strong>4</strong> <strong>4</strong><strong> │ </strong>  
  
</code></pre>{% endraw %}

### Valid

```jsx
const boolExpr3 = true;
const boolExpr4 = false;
const r5 = !(boolExpr1 && boolExpr2);
const boolExpr5 = true;
const boolExpr6 = false;
const r6 = !!boolExpr1 || !!boolExpr2;
```

