---
title: Lint Rule noConditionalAssignment
parent: lint/rules/index
---

# noConditionalAssignment (since v11.0.0)

> This rule is recommended by Rome.

Disallow assignment operators in conditional expressions.

## Examples

### Invalid

```jsx
var x;
if (x = 0) {
    var b = 1;
}
```

<pre class="language-text"><code class="language-text">nursery/noConditionalAssignment.js:2:5 <a href="https://docs.rome.tools/lint/rules/noConditionalAssignment">lint/nursery/noConditionalAssignment</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Expected a conditional expression and instead saw an assignment.</span>
  
    <strong>1 │ </strong>var x;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>if (x = 0) {
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>    var b = 1;
    <strong>4 │ </strong>}
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Did you mean '==='?</span>
  
<strong>  </strong><strong>  2 │ </strong>if<span style="opacity: 0.8;">·</span>(x<span style="opacity: 0.8;">·</span>=<span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;">=</span><span style="opacity: 0.8;">·</span>0)<span style="opacity: 0.8;">·</span>{
<strong>  </strong><strong>    │ </strong>       <span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span>     
</code></pre>

```jsx
function setHeight(someNode) {
    "use strict";
    do {
        someNode.height = "100px";
    } while (someNode = someNode.parentNode);
}
```

<pre class="language-text"><code class="language-text">nursery/noConditionalAssignment.js:5:14 <a href="https://docs.rome.tools/lint/rules/noConditionalAssignment">lint/nursery/noConditionalAssignment</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Expected a conditional expression and instead saw an assignment.</span>
  
    <strong>3 │ </strong>    do {
    <strong>4 │ </strong>        someNode.height = &quot;100px&quot;;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>5 │ </strong>    } while (someNode = someNode.parentNode);
   <strong>   │ </strong>             <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>6 │ </strong>}
    <strong>7 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Did you mean '==='?</span>
  
<strong>  </strong><strong>  5 │ </strong><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span>}<span style="opacity: 0.8;">·</span>while<span style="opacity: 0.8;">·</span>(someNode<span style="opacity: 0.8;">·</span>=<span style="color: MediumSeaGreen;">=</span><span style="color: MediumSeaGreen;">=</span><span style="opacity: 0.8;">·</span>someNode.parentNode);
<strong>  </strong><strong>    │ </strong>                       <span style="color: MediumSeaGreen;">+</span><span style="color: MediumSeaGreen;">+</span>                      
</code></pre>

### Valid

```jsx
var x;
if (x === 0) {
    var b = 1;
}
```

```jsx
function setHeight(someNode) {
    "use strict";
    do {
        someNode.height = "100px";
    } while ((someNode = someNode.parentNode) !== null);
}
```

```
```

