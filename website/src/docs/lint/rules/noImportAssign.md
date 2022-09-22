---
title: Lint Rule noImportAssign
layout: layouts/rule.liquid
---

# noImportAssign (since v0.9.0)

> This rule is recommended by Rome.

Disallow assigning to imported bindings

## Examples

### Invalid

```jsx
import x from "y";
x = 1;
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noImportAssign.js:2:1 <a href="https://rome.tools/docs/lint/rules/noImportAssign">lint/correctness/noImportAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The imported variable </span><span style="color: Tomato;"><strong>x</strong></span><span style="color: Tomato;"> is read-only</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noImportAssign.js:2:1
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">x</span> = 1;
    <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The variable is imported here</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noImportAssign.js:1:8
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> import <span style="color: Tomato;">x</span> from &quot;y&quot;;
    <span style="color: rgb(38, 148, 255);">│</span>        <span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Use a local variable instead of reassigning an import.</span>
  
</code></pre>{% endraw %}

```jsx
import y from "y";
[y] = 1;
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noImportAssign.js:2:2 <a href="https://rome.tools/docs/lint/rules/noImportAssign">lint/correctness/noImportAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The imported variable </span><span style="color: Tomato;"><strong>y</strong></span><span style="color: Tomato;"> is read-only</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noImportAssign.js:2:2
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> [<span style="color: Tomato;">y</span>] = 1;
    <span style="color: rgb(38, 148, 255);">│</span>  <span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The variable is imported here</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noImportAssign.js:1:8
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> import <span style="color: Tomato;">y</span> from &quot;y&quot;;
    <span style="color: rgb(38, 148, 255);">│</span>        <span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Use a local variable instead of reassigning an import.</span>
  
</code></pre>{% endraw %}

```jsx
import z from "y";
({ z } = 1); /// ```
```js,expect_diagnostic
import a from "y";
[...a] = 1;
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noImportAssign.js:3:4 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">unterminated template literal</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noImportAssign.js:3:4
    <span style="color: rgb(38, 148, 255);">│</span>  
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">3</span> <span style="color: rgb(38, 148, 255);">│</span>   ```<span style="color: Tomato;">j</span><span style="color: Tomato;">s</span><span style="color: Tomato;">,</span><span style="color: Tomato;">e</span><span style="color: Tomato;">x</span><span style="color: Tomato;">p</span><span style="color: Tomato;">e</span><span style="color: Tomato;">c</span><span style="color: Tomato;">t</span><span style="color: Tomato;">_</span><span style="color: Tomato;">d</span><span style="color: Tomato;">i</span><span style="color: Tomato;">a</span><span style="color: Tomato;">g</span><span style="color: Tomato;">n</span><span style="color: Tomato;">o</span><span style="color: Tomato;">s</span><span style="color: Tomato;">t</span><span style="color: Tomato;">i</span><span style="color: Tomato;">c</span>
    <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">┌</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">^</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">4</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">│</span> <span style="color: Tomato;">i</span><span style="color: Tomato;">m</span><span style="color: Tomato;">p</span><span style="color: Tomato;">o</span><span style="color: Tomato;">r</span><span style="color: Tomato;">t</span><span style="color: Tomato;"> </span><span style="color: Tomato;">a</span><span style="color: Tomato;"> </span><span style="color: Tomato;">f</span><span style="color: Tomato;">r</span><span style="color: Tomato;">o</span><span style="color: Tomato;">m</span><span style="color: Tomato;"> </span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">y</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">;</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">5</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">│</span> <span style="color: Tomato;">[</span><span style="color: Tomato;">.</span><span style="color: Tomato;">.</span><span style="color: Tomato;">.</span><span style="color: Tomato;">a</span><span style="color: Tomato;">]</span><span style="color: Tomato;"> </span><span style="color: Tomato;">=</span><span style="color: Tomato;"> </span><span style="color: Tomato;">1</span><span style="color: Tomato;">;</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">6</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">│</span> 
    <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">└</span><span style="color: Tomato;">^</span>
  
</code></pre>{% endraw %}

```jsx
import b from "y";
({ ...b } = 1);
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noImportAssign.js:2:7 <a href="https://rome.tools/docs/lint/rules/noImportAssign">lint/correctness/noImportAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The imported variable </span><span style="color: Tomato;"><strong>b</strong></span><span style="color: Tomato;"> is read-only</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noImportAssign.js:2:7
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> ({ ...<span style="color: Tomato;">b</span> } = 1);
    <span style="color: rgb(38, 148, 255);">│</span>       <span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The variable is imported here</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noImportAssign.js:1:8
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> import <span style="color: Tomato;">b</span> from &quot;y&quot;;
    <span style="color: rgb(38, 148, 255);">│</span>        <span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Use a local variable instead of reassigning an import.</span>
  
</code></pre>{% endraw %}

```jsx
import c from "y";
for (c in y) {};
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noImportAssign.js:2:6 <a href="https://rome.tools/docs/lint/rules/noImportAssign">lint/correctness/noImportAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The imported variable </span><span style="color: Tomato;"><strong>c</strong></span><span style="color: Tomato;"> is read-only</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noImportAssign.js:2:6
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> for (<span style="color: Tomato;">c</span> in y) {};
    <span style="color: rgb(38, 148, 255);">│</span>      <span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The variable is imported here</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noImportAssign.js:1:8
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> import <span style="color: Tomato;">c</span> from &quot;y&quot;;
    <span style="color: rgb(38, 148, 255);">│</span>        <span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Use a local variable instead of reassigning an import.</span>
  
</code></pre>{% endraw %}

```jsx
import d from "y";
d += 1;
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noImportAssign.js:2:1 <a href="https://rome.tools/docs/lint/rules/noImportAssign">lint/correctness/noImportAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The imported variable </span><span style="color: Tomato;"><strong>d</strong></span><span style="color: Tomato;"> is read-only</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noImportAssign.js:2:1
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">d</span> += 1;
    <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The variable is imported here</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noImportAssign.js:1:8
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> import <span style="color: Tomato;">d</span> from &quot;y&quot;;
    <span style="color: rgb(38, 148, 255);">│</span>        <span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Use a local variable instead of reassigning an import.</span>
  
</code></pre>{% endraw %}

```jsx
import * as e from "y";
e = 1;
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noImportAssign.js:2:1 <a href="https://rome.tools/docs/lint/rules/noImportAssign">lint/correctness/noImportAssign</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The imported variable </span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"> is read-only</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noImportAssign.js:2:1
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">e</span> = 1;
    <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The variable is imported here</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> correctness/noImportAssign.js:1:13
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> import * as <span style="color: Tomato;">e</span> from &quot;y&quot;;
    <span style="color: rgb(38, 148, 255);">│</span>             <span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Use a local variable instead of reassigning an import.</span>
  
</code></pre>{% endraw %}

