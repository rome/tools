---
title: Lint Rule noImportAssign
layout: layouts/rule.liquid
---

# noImportAssign (since v0.7.0)

Disallow assigning to imported bindings

## Examples

### Invalid

```jsx
import x from "y";
x = 1;
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;">js/noImportAssign</span><span style="color: Orange;">]</span><em>: </em><em>The imported variable </em><em><em>x</em></em><em> is read-only</em>
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

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;">js/noImportAssign</span><span style="color: Orange;">]</span><em>: </em><em>The imported variable </em><em><em>y</em></em><em> is read-only</em>
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
({ z } = 1);
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;">js/noImportAssign</span><span style="color: Orange;">]</span><em>: </em><em>The imported variable </em><em><em>z</em></em><em> is read-only</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noImportAssign.js:2:4
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> import z from &quot;y&quot;;
  <span style="color: rgb(38, 148, 255);">│</span>        <span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">The variable is imported here</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> ({ z } = 1);
  <span style="color: rgb(38, 148, 255);">│</span>    <span style="color: rgb(38, 148, 255);">-</span>

=  note: Use a local variable instead of reassigning an import.

</code></pre>{% endraw %}

```jsx
import a from "y";
[...a] = 1;
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;">js/noImportAssign</span><span style="color: Orange;">]</span><em>: </em><em>The imported variable </em><em><em>a</em></em><em> is read-only</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noImportAssign.js:2:5
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> import a from &quot;y&quot;;
  <span style="color: rgb(38, 148, 255);">│</span>        <span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">The variable is imported here</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> [...a] = 1;
  <span style="color: rgb(38, 148, 255);">│</span>     <span style="color: rgb(38, 148, 255);">-</span>

=  note: Use a local variable instead of reassigning an import.

</code></pre>{% endraw %}

```jsx
import b from "y";
({ ...b } = 1);
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;">js/noImportAssign</span><span style="color: Orange;">]</span><em>: </em><em>The imported variable </em><em><em>b</em></em><em> is read-only</em>
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

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;">js/noImportAssign</span><span style="color: Orange;">]</span><em>: </em><em>The imported variable </em><em><em>c</em></em><em> is read-only</em>
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

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;">js/noImportAssign</span><span style="color: Orange;">]</span><em>: </em><em>The imported variable </em><em><em>d</em></em><em> is read-only</em>
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

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;">js/noImportAssign</span><span style="color: Orange;">]</span><em>: </em><em>The imported variable </em><em><em>e</em></em><em> is read-only</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> js/noImportAssign.js:2:1
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> import * as e from &quot;y&quot;;
  <span style="color: rgb(38, 148, 255);">│</span>             <span style="color: rgb(38, 148, 255);">-</span> <span style="color: rgb(38, 148, 255);">The variable is imported here</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> e = 1;
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">-</span>

=  note: Use a local variable instead of reassigning an import.

</code></pre>{% endraw %}

