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

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><span style="color: Tomato;">[</span><span style="color: Tomato;"><a href="https://rome.tools/docs/lint/rules/noShoutyConstants/">style/noShoutyConstants</a></span><span style="color: Tomato;">]</span><em>: </em><em>Redundant constant declaration.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> style/noShoutyConstants.js:1:7
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> const <span style="color: Tomato;">F</span><span style="color: Tomato;">O</span><span style="color: Tomato;">O</span><span style="color: Tomato;"> </span><span style="color: Tomato;">=</span><span style="color: Tomato;"> </span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">F</span><span style="color: Tomato;">O</span><span style="color: Tomato;">O</span><span style="color: Tomato;">&quot;</span>;
  <span style="color: rgb(38, 148, 255);">│</span>       <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> console.log(FOO);
  <span style="color: rgb(38, 148, 255);">│</span>             <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">Used here.</span>

<span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Use the constant value directly</span>
    | <span style="color: rgb(38, 148, 255);">@@ -1,2 +1,2 @@</span>
0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">const FOO = &quot;FOO&quot;;</span>
1   | <span style="color: Tomato;">- </span><span style="color: Tomato;">console.log(FOO);</span>
  0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;"></span>
  1 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">console.log(&quot;FOO&quot;);</span>

=  note: You should avoid declaring constants with a string that's the same
    value as the variable name. It introduces a level of unnecessary
    indirection when it's only two additional characters to inline.

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

