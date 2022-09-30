---
title: Lint Rule noShoutyConstants
layout: layouts/rule.liquid
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

{% raw %}<pre class="language-text"><code class="language-text">style/noShoutyConstants.js:1:7 <a href="https://rome.tools/docs/lint/rules/noShoutyConstants">lint/style/noShoutyConstants</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

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
  
      | <span style="color: rgb(38, 148, 255);">@@ -1,2 +1,2 @@</span>
  0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">const FOO = &quot;FOO&quot;;</span>
  1   | <span style="color: Tomato;">- </span><span style="color: Tomato;">console.log(FOO);</span>
    0 | <span style="color: MediumSeaGreen;">+ </span>
    1 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">console.log(&quot;FOO&quot;);</span>
  
</code></pre>{% endraw %}

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

