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

{% raw %}<pre class="language-text"><code class="language-text">nursery/noUselessFragments.js:1:1 <a href="https://rome.tools/docs/lint/rules/noUselessFragments">lint/nursery/noUselessFragments</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Avoid using unnecessary </span><span style="color: Orange;"><strong>Fragment</strong></span><span style="color: Orange;">.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noUselessFragments.js:1:1
    <span style="color: rgb(38, 148, 255);">│</span>  
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">┌</span> <span style="color: Tomato;">&lt;</span><span style="color: Tomato;">&gt;</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">│</span> <span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">3</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">│</span> <span style="color: Tomato;">&lt;</span><span style="color: Tomato;">/</span><span style="color: Tomato;">&gt;</span>
    <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">└</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">^</span>
  
</code></pre>{% endraw %}

```jsx
<React.Fragment>
foo
</React.Fragment>
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noUselessFragments.js:1:1 <a href="https://rome.tools/docs/lint/rules/noUselessFragments">lint/nursery/noUselessFragments</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Avoid using unnecessary </span><span style="color: Orange;"><strong>Fragment</strong></span><span style="color: Orange;">.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noUselessFragments.js:1:1
    <span style="color: rgb(38, 148, 255);">│</span>  
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">┌</span> <span style="color: Tomato;">&lt;</span><span style="color: Tomato;">R</span><span style="color: Tomato;">e</span><span style="color: Tomato;">a</span><span style="color: Tomato;">c</span><span style="color: Tomato;">t</span><span style="color: Tomato;">.</span><span style="color: Tomato;">F</span><span style="color: Tomato;">r</span><span style="color: Tomato;">a</span><span style="color: Tomato;">g</span><span style="color: Tomato;">m</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">t</span><span style="color: Tomato;">&gt;</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">│</span> <span style="color: Tomato;">f</span><span style="color: Tomato;">o</span><span style="color: Tomato;">o</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">3</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">│</span> <span style="color: Tomato;">&lt;</span><span style="color: Tomato;">/</span><span style="color: Tomato;">R</span><span style="color: Tomato;">e</span><span style="color: Tomato;">a</span><span style="color: Tomato;">c</span><span style="color: Tomato;">t</span><span style="color: Tomato;">.</span><span style="color: Tomato;">F</span><span style="color: Tomato;">r</span><span style="color: Tomato;">a</span><span style="color: Tomato;">g</span><span style="color: Tomato;">m</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">t</span><span style="color: Tomato;">&gt;</span>
    <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">└</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">─</span><span style="color: Tomato;">^</span>
  
</code></pre>{% endraw %}

```jsx
<></>
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noUselessFragments.js:1:1 <a href="https://rome.tools/docs/lint/rules/noUselessFragments">lint/nursery/noUselessFragments</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Avoid using unnecessary </span><span style="color: Orange;"><strong>Fragment</strong></span><span style="color: Orange;">.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noUselessFragments.js:1:1
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">&lt;</span><span style="color: Tomato;">&gt;</span><span style="color: Tomato;">&lt;</span><span style="color: Tomato;">/</span><span style="color: Tomato;">&gt;</span>
    <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
</code></pre>{% endraw %}

