---
title: Lint Rule noShoutyConstants
---

# noShoutyConstants (since v0.7.0)

> This rule is recommended by Rome.

Disallow the use of constants which its value is the upper-case version of its name.

## Examples

### Invalid

```jsx
const FOO = "FOO";
console.log(FOO);
```

<pre class="language-text"><code class="language-text">style/noShoutyConstants.js:1:7 <a href="https://docs.rome.tools/lint/rules/noShoutyConstants">lint/style/noShoutyConstants</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Redundant constant declaration.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>const FOO = &quot;FOO&quot;;
   <strong>   │ </strong>      <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>console.log(FOO);
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Used here.</span>
  
    <strong>1 │ </strong>const FOO = &quot;FOO&quot;;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>console.log(FOO);
   <strong>   │ </strong>            <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">You should avoid declaring constants with a string that's the same
</span><span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">    value as the variable name. It introduces a level of unnecessary
</span><span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">    indirection when it's only two additional characters to inline.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Use the constant value directly</span>
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>c</strong></span><span style="color: Tomato;"><strong>o</strong></span><span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;"><strong>s</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>F</strong></span><span style="color: Tomato;"><strong>O</strong></span><span style="color: Tomato;"><strong>O</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>=</strong></span><span style="color: Tomato;"><span style="opacity: 0.8;"><strong>·</strong></span></span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;"><strong>F</strong></span><span style="color: Tomato;"><strong>O</strong></span><span style="color: Tomato;"><strong>O</strong></span><span style="color: Tomato;"><strong>&quot;</strong></span><span style="color: Tomato;"><strong>;</strong></span>
    <strong>2</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;">c</span><span style="color: Tomato;">o</span><span style="color: Tomato;">n</span><span style="color: Tomato;">s</span><span style="color: Tomato;">o</span><span style="color: Tomato;">l</span><span style="color: Tomato;">e</span><span style="color: Tomato;">.</span><span style="color: Tomato;">l</span><span style="color: Tomato;">o</span><span style="color: Tomato;">g</span><span style="color: Tomato;">(</span><span style="color: Tomato;">F</span><span style="color: Tomato;">O</span><span style="color: Tomato;">O</span><span style="color: Tomato;">)</span><span style="color: Tomato;">;</span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> 
      <strong>2</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;">c</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">n</span><span style="color: MediumSeaGreen;">s</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">l</span><span style="color: MediumSeaGreen;">e</span><span style="color: MediumSeaGreen;">.</span><span style="color: MediumSeaGreen;">l</span><span style="color: MediumSeaGreen;">o</span><span style="color: MediumSeaGreen;">g</span><span style="color: MediumSeaGreen;">(</span><span style="color: MediumSeaGreen;"><strong>&quot;</strong></span><span style="color: MediumSeaGreen;">F</span><span style="color: MediumSeaGreen;">O</span><span style="color: MediumSeaGreen;">O</span><span style="color: MediumSeaGreen;"><strong>&quot;</strong></span><span style="color: MediumSeaGreen;">)</span><span style="color: MediumSeaGreen;">;</span>
    <strong>3</strong> <strong>3</strong><strong> │ </strong>  
  
</code></pre>

### Valid

```jsx
let FOO = "FOO";
console.log(FOO);
```

```jsx
export const FOO = "FOO";
console.log(FOO);
```

```jsx
function f(FOO = "FOO") {
    return FOO;
}
```

