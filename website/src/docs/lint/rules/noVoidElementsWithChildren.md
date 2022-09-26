---
title: Lint Rule noVoidElementsWithChildren
layout: layouts/rule.liquid
---

# noVoidElementsWithChildren (since v0.10.0)

This rules prevents void elements (AKA self-closing elements) from having children.

## Examples

### Invalid

```jsx
<br>invalid child</br>
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noVoidElementsWithChildren.js:1:1 <a href="https://rome.tools/docs/lint/rules/noVoidElementsWithChildren">lint/nursery/noVoidElementsWithChildren</a> ━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;"><strong>br</strong></span><span style="color: Orange;"> is a void element tag and must not have </span><span style="color: Orange;"><strong>children</strong></span><span style="color: Orange;">.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noVoidElementsWithChildren.js:1:1
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">&lt;</span><span style="color: Tomato;">b</span><span style="color: Tomato;">r</span><span style="color: Tomato;">&gt;</span><span style="color: Tomato;">i</span><span style="color: Tomato;">n</span><span style="color: Tomato;">v</span><span style="color: Tomato;">a</span><span style="color: Tomato;">l</span><span style="color: Tomato;">i</span><span style="color: Tomato;">d</span><span style="color: Tomato;"> </span><span style="color: Tomato;">c</span><span style="color: Tomato;">h</span><span style="color: Tomato;">i</span><span style="color: Tomato;">l</span><span style="color: Tomato;">d</span><span style="color: Tomato;">&lt;</span><span style="color: Tomato;">/</span><span style="color: Tomato;">b</span><span style="color: Tomato;">r</span><span style="color: Tomato;">&gt;</span>
    <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
</code></pre>{% endraw %}

```jsx
<img alt="some text" children={"some child"} />
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noVoidElementsWithChildren.js:1:1 <a href="https://rome.tools/docs/lint/rules/noVoidElementsWithChildren">lint/nursery/noVoidElementsWithChildren</a> ━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;"><strong>img</strong></span><span style="color: Orange;"> is a void element tag and must not have </span><span style="color: Orange;"><strong>children</strong></span><span style="color: Orange;">.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noVoidElementsWithChildren.js:1:1
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">&lt;</span><span style="color: Tomato;">i</span><span style="color: Tomato;">m</span><span style="color: Tomato;">g</span><span style="color: Tomato;"> </span><span style="color: Tomato;">a</span><span style="color: Tomato;">l</span><span style="color: Tomato;">t</span><span style="color: Tomato;">=</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">s</span><span style="color: Tomato;">o</span><span style="color: Tomato;">m</span><span style="color: Tomato;">e</span><span style="color: Tomato;"> </span><span style="color: Tomato;">t</span><span style="color: Tomato;">e</span><span style="color: Tomato;">x</span><span style="color: Tomato;">t</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;"> </span><span style="color: Tomato;">c</span><span style="color: Tomato;">h</span><span style="color: Tomato;">i</span><span style="color: Tomato;">l</span><span style="color: Tomato;">d</span><span style="color: Tomato;">r</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">=</span><span style="color: Tomato;">{</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">s</span><span style="color: Tomato;">o</span><span style="color: Tomato;">m</span><span style="color: Tomato;">e</span><span style="color: Tomato;"> </span><span style="color: Tomato;">c</span><span style="color: Tomato;">h</span><span style="color: Tomato;">i</span><span style="color: Tomato;">l</span><span style="color: Tomato;">d</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">}</span><span style="color: Tomato;"> </span><span style="color: Tomato;">/</span><span style="color: Tomato;">&gt;</span>
    <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
</code></pre>{% endraw %}

