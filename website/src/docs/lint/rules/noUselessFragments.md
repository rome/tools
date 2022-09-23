---
title: Lint Rule noUselessFragments
layout: layouts/rule.liquid
---

# noUselessFragments (since v0.10.0)

Disallow unnecessary fragments

## Examples

### Invalid

```jsx
<>
foo
</>
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noUselessFragments/">nursery/noUselessFragments</a></span><span style="color: Orange;">]</span><strong>: </strong><strong>Avoid using unnecessary </strong><strong><strong>Fragment</strong></strong><strong>.</strong>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noUselessFragments.js:1:1
  <span style="color: rgb(38, 148, 255);">│</span>  
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">┌</span> &lt;&gt;
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">│</span> foo
<span style="color: rgb(38, 148, 255);">3</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">│</span> &lt;/&gt;
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">└</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">'</span>

</code></pre>{% endraw %}

```jsx
<React.Fragment>
foo
</React.Fragment>
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noUselessFragments/">nursery/noUselessFragments</a></span><span style="color: Orange;">]</span><strong>: </strong><strong>Avoid using unnecessary </strong><strong><strong>Fragment</strong></strong><strong>.</strong>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noUselessFragments.js:1:1
  <span style="color: rgb(38, 148, 255);">│</span>  
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">┌</span> &lt;React.Fragment&gt;
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">│</span> foo
<span style="color: rgb(38, 148, 255);">3</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">│</span> &lt;/React.Fragment&gt;
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">└</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">─</span><span style="color: rgb(38, 148, 255);">'</span>

</code></pre>{% endraw %}

```jsx
<></>
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noUselessFragments/">nursery/noUselessFragments</a></span><span style="color: Orange;">]</span><strong>: </strong><strong>Avoid using unnecessary </strong><strong><strong>Fragment</strong></strong><strong>.</strong>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noUselessFragments.js:1:1
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> &lt;&gt;&lt;/&gt;
  <span style="color: rgb(38, 148, 255);">│</span> <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

</code></pre>{% endraw %}

