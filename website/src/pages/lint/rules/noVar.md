---
title: Lint Rule noVar
parent: lint/rules/index
---

# noVar (since v11.0.0)

> This rule is recommended by Rome.

Disallow the use of `var`

ECMAScript 6 allows programmers to create variables with block scope instead of function scope using the let and const keywords. Block scope is common in many other programming languages and helps programmers avoid mistakes.

Source: https://eslint.org/docs/latest/rules/no-var

## Examples

### Invalid

```jsx
var foo = 1;
```

<pre class="language-text"><code class="language-text">nursery/noVar.js:1:1 <a href="https://docs.rome.tools/lint/rules/noVar">lint/nursery/noVar</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Use </span><span style="color: Tomato;"><strong>let</strong></span><span style="color: Tomato;"> or </span><span style="color: Tomato;"><strong>const</strong></span><span style="color: Tomato;"> instead of </span><span style="color: Tomato;"><strong>var</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>var foo = 1;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">A variable declared with </span><span style="color: rgb(38, 148, 255);"><strong>var</strong></span><span style="color: rgb(38, 148, 255);"> is accessible in the whole module. Thus, the variable can be accessed before its initialization and outside the block where it is declared.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">See </span><span style="color: rgb(38, 148, 255);"><a href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/var">MDN web docs</a></span><span style="color: rgb(38, 148, 255);"> for more details.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Use '</span><span style="color: rgb(38, 148, 255);"><strong>const</strong></span><span style="color: rgb(38, 148, 255);">' instead.</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>v</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">=</span><span style="color: Tomato;"><span style="opacity: 0.8;">·</span></span><span style="color: Tomato;">1</span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>c</strong></span><span style="color: MediumSeaGreen;"><strong>o</strong></span><span style="color: MediumSeaGreen;"><strong>n</strong></span><span style="color: MediumSeaGreen;"><strong>s</strong></span><span style="color: MediumSeaGreen;"><strong>t</strong></span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">f</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;"><span style="opacity: 0.8;">·</span></span><span style="color: MediumSeaGreen;">1</span><span style="color: MediumSeaGreen;">;</span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  
  
</code></pre>

### Valid

```jsx
const foo = 1;
let bar = 1;
```

