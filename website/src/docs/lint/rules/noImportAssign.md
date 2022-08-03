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

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noImportAssign/">js/noImportAssign</a></span><span style="color: Orange;">]</span><em>: </em><em>The imported variable </em><em><em>x</em></em><em> is read-only</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noImportAssign.js:2:1
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> import x from &quot;y&quot;;
  <span style="color: rgb(38, 148, 255);">│</span>        <span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">The variable is imported here</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> x = 1;
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">-</span>

=  note: Use a local variable instead of reassigning an import.

</code></pre>{% endraw %}

```jsx
import y from "y";
[y] = 1;
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noImportAssign/">js/noImportAssign</a></span><span style="color: Orange;">]</span><em>: </em><em>The imported variable </em><em><em>y</em></em><em> is read-only</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noImportAssign.js:2:2
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> import y from &quot;y&quot;;
  <span style="color: rgb(38, 148, 255);">│</span>        <span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">The variable is imported here</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> [y] = 1;
  <span style="color: rgb(38, 148, 255);">│</span>  <span style="color: rgb(38, 148, 255);">-</span>

=  note: Use a local variable instead of reassigning an import.

</code></pre>{% endraw %}

```jsx
import z from "y";
({ z } = 1); /// ```
```js,expect_diagnostic
import a from "y";
[...a] = 1;
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Tomato;">error</span><em>: </em><em>unterminated template literal</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noImportAssign.js:3:4
  <span style="color: rgb(38, 148, 255);">│</span>  
<span style="color: rgb(38, 148, 255);">3</span> <span style="color: rgb(38, 148, 255);">│</span>   ```<span style="color: Tomato;">j</span><span style="color: Tomato;">s</span><span style="color: Tomato;">,</span><span style="color: Tomato;">e</span><span style="color: Tomato;">x</span><span style="color: Tomato;">p</span><span style="color: Tomato;">e</span><span style="color: Tomato;">c</span><span style="color: Tomato;">t</span><span style="color: Tomato;">_</span><span style="color: Tomato;">d</span><span style="color: Tomato;">i</span><span style="color: Tomato;">a</span><span style="color: Tomato;">g</span><span style="color: Tomato;">n</span><span style="color: Tomato;">o</span><span style="color: Tomato;">s</span><span style="color: Tomato;">t</span><span style="color: Tomato;">i</span><span style="color: Tomato;">c</span>
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">┌</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">^</span>
<span style="color: rgb(38, 148, 255);">4</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">│</span> <span style="color: Tomato;">i</span><span style="color: Tomato;">m</span><span style="color: Tomato;">p</span><span style="color: Tomato;">o</span><span style="color: Tomato;">r</span><span style="color: Tomato;">t</span><span style="color: Tomato;"> </span><span style="color: Tomato;">a</span><span style="color: Tomato;"> </span><span style="color: Tomato;">f</span><span style="color: Tomato;">r</span><span style="color: Tomato;">o</span><span style="color: Tomato;">m</span><span style="color: Tomato;"> </span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">y</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">;</span>
<span style="color: rgb(38, 148, 255);">5</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">│</span> <span style="color: Tomato;">[</span><span style="color: Tomato;">.</span><span style="color: Tomato;">.</span><span style="color: Tomato;">.</span><span style="color: Tomato;">a</span><span style="color: Tomato;">]</span><span style="color: Tomato;"> </span><span style="color: Tomato;">=</span><span style="color: Tomato;"> </span><span style="color: Tomato;">1</span><span style="color: Tomato;">;</span>
<span style="color: rgb(38, 148, 255);">6</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">│</span> 
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">└</span><span style="color: Tomato;">^</span>

</code></pre>{% endraw %}

```jsx
import b from "y";
({ ...b } = 1);
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noImportAssign/">js/noImportAssign</a></span><span style="color: Orange;">]</span><em>: </em><em>The imported variable </em><em><em>b</em></em><em> is read-only</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noImportAssign.js:2:7
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> import b from &quot;y&quot;;
  <span style="color: rgb(38, 148, 255);">│</span>        <span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">The variable is imported here</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> ({ ...b } = 1);
  <span style="color: rgb(38, 148, 255);">│</span>       <span style="color: rgb(38, 148, 255);">-</span>

=  note: Use a local variable instead of reassigning an import.

</code></pre>{% endraw %}

```jsx
import c from "y";
for (c in y) {};
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noImportAssign/">js/noImportAssign</a></span><span style="color: Orange;">]</span><em>: </em><em>The imported variable </em><em><em>c</em></em><em> is read-only</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noImportAssign.js:2:6
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> import c from &quot;y&quot;;
  <span style="color: rgb(38, 148, 255);">│</span>        <span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">The variable is imported here</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> for (c in y) {};
  <span style="color: rgb(38, 148, 255);">│</span>      <span style="color: rgb(38, 148, 255);">-</span>

=  note: Use a local variable instead of reassigning an import.

</code></pre>{% endraw %}

```jsx
import d from "y";
d += 1;
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noImportAssign/">js/noImportAssign</a></span><span style="color: Orange;">]</span><em>: </em><em>The imported variable </em><em><em>d</em></em><em> is read-only</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noImportAssign.js:2:1
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> import d from &quot;y&quot;;
  <span style="color: rgb(38, 148, 255);">│</span>        <span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">The variable is imported here</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> d += 1;
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">-</span>

=  note: Use a local variable instead of reassigning an import.

</code></pre>{% endraw %}

```jsx
import * as e from "y";
e = 1;
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noImportAssign/">js/noImportAssign</a></span><span style="color: Orange;">]</span><em>: </em><em>The imported variable </em><em><em>e</em></em><em> is read-only</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noImportAssign.js:2:1
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> import * as e from &quot;y&quot;;
  <span style="color: rgb(38, 148, 255);">│</span>             <span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">The variable is imported here</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> e = 1;
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">-</span>

=  note: Use a local variable instead of reassigning an import.

</code></pre>{% endraw %}

