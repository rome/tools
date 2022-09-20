---
title: Lint Rule noUselessFragment
layout: layouts/rule.liquid
---

# noUselessFragment (since v0.10.0)

Disallow unnecessary fragments

## Examples

### Invalid

```jsx
<>
foo
</>
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noUselessFragment/">nursery/noUselessFragment</a></span><span style="color: Orange;">]</span><em>: </em><em>Avoid using unnecessary </em><em><em>Fragment</em></em><em>.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noUselessFragment.js:1:1
  <span style="color: rgb(38, 148, 255);">│</span>  
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">┌</span> &lt;&gt;
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">│</span> foo
<span style="color: rgb(38, 148, 255);">3</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">│</span> &lt;/&gt;
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">└</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">'</span>

</code></pre>{% endraw %}

```jsx
<></>
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noUselessFragment/">nursery/noUselessFragment</a></span><span style="color: Orange;">]</span><em>: </em><em>Avoid using unnecessary </em><em><em>Fragment</em></em><em>.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noUselessFragment.js:1:1
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> &lt;&gt;&lt;/&gt;
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

</code></pre>{% endraw %}

