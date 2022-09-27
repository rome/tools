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

{% raw %}<pre class="language-text"><code class="language-text">nursery/noVoidElementsWithChildren.js:1:1 <a href="https://rome.tools/docs/lint/rules/noVoidElementsWithChildren">lint/nursery/noVoidElementsWithChildren</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;"><strong>br</strong></span><span style="color: Orange;"> is a void element tag and must not have </span><span style="color: Orange;"><strong>children</strong></span><span style="color: Orange;">.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noVoidElementsWithChildren.js:1:1
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">&lt;</span><span style="color: Tomato;">b</span><span style="color: Tomato;">r</span><span style="color: Tomato;">&gt;</span><span style="color: Tomato;">i</span><span style="color: Tomato;">n</span><span style="color: Tomato;">v</span><span style="color: Tomato;">a</span><span style="color: Tomato;">l</span><span style="color: Tomato;">i</span><span style="color: Tomato;">d</span><span style="color: Tomato;"> </span><span style="color: Tomato;">c</span><span style="color: Tomato;">h</span><span style="color: Tomato;">i</span><span style="color: Tomato;">l</span><span style="color: Tomato;">d</span><span style="color: Tomato;">&lt;</span><span style="color: Tomato;">/</span><span style="color: Tomato;">b</span><span style="color: Tomato;">r</span><span style="color: Tomato;">&gt;</span>
    <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove the </span><span style="color: rgb(38, 148, 255);"><strong>children</strong></span><span style="color: rgb(38, 148, 255);">.</span>
  
      | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
  0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">&lt;br&gt;invalid child&lt;/br&gt;</span>
    0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">&lt;br/&gt;</span>
  
</code></pre>{% endraw %}

```jsx
<img alt="some text" children={"some child"} />
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noVoidElementsWithChildren.js:1:1 <a href="https://rome.tools/docs/lint/rules/noVoidElementsWithChildren">lint/nursery/noVoidElementsWithChildren</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;"><strong>img</strong></span><span style="color: Orange;"> is a void element tag and must not have </span><span style="color: Orange;"><strong>children</strong></span><span style="color: Orange;">.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noVoidElementsWithChildren.js:1:1
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">&lt;</span><span style="color: Tomato;">i</span><span style="color: Tomato;">m</span><span style="color: Tomato;">g</span><span style="color: Tomato;"> </span><span style="color: Tomato;">a</span><span style="color: Tomato;">l</span><span style="color: Tomato;">t</span><span style="color: Tomato;">=</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">s</span><span style="color: Tomato;">o</span><span style="color: Tomato;">m</span><span style="color: Tomato;">e</span><span style="color: Tomato;"> </span><span style="color: Tomato;">t</span><span style="color: Tomato;">e</span><span style="color: Tomato;">x</span><span style="color: Tomato;">t</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;"> </span><span style="color: Tomato;">c</span><span style="color: Tomato;">h</span><span style="color: Tomato;">i</span><span style="color: Tomato;">l</span><span style="color: Tomato;">d</span><span style="color: Tomato;">r</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">=</span><span style="color: Tomato;">{</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">s</span><span style="color: Tomato;">o</span><span style="color: Tomato;">m</span><span style="color: Tomato;">e</span><span style="color: Tomato;"> </span><span style="color: Tomato;">c</span><span style="color: Tomato;">h</span><span style="color: Tomato;">i</span><span style="color: Tomato;">l</span><span style="color: Tomato;">d</span><span style="color: Tomato;">&quot;</span><span style="color: Tomato;">}</span><span style="color: Tomato;"> </span><span style="color: Tomato;">/</span><span style="color: Tomato;">&gt;</span>
    <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove the </span><span style="color: rgb(38, 148, 255);"><strong>children</strong></span><span style="color: rgb(38, 148, 255);">.</span>
  
      | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
  0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">&lt;img alt=&quot;some text&quot; children={&quot;some child&quot;} /&gt;</span>
    0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">&lt;img alt=&quot;some text&quot; /&gt;</span>
  
</code></pre>{% endraw %}

```jsx
React.createElement('img', {}, 'child')
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noVoidElementsWithChildren.js:1:1 <a href="https://rome.tools/docs/lint/rules/noVoidElementsWithChildren">lint/nursery/noVoidElementsWithChildren</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;"><strong>img</strong></span><span style="color: Orange;"> is a void element tag and must not have </span><span style="color: Orange;"><strong>children</strong></span><span style="color: Orange;">.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noVoidElementsWithChildren.js:1:1
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">R</span><span style="color: Tomato;">e</span><span style="color: Tomato;">a</span><span style="color: Tomato;">c</span><span style="color: Tomato;">t</span><span style="color: Tomato;">.</span><span style="color: Tomato;">c</span><span style="color: Tomato;">r</span><span style="color: Tomato;">e</span><span style="color: Tomato;">a</span><span style="color: Tomato;">t</span><span style="color: Tomato;">e</span><span style="color: Tomato;">E</span><span style="color: Tomato;">l</span><span style="color: Tomato;">e</span><span style="color: Tomato;">m</span><span style="color: Tomato;">e</span><span style="color: Tomato;">n</span><span style="color: Tomato;">t</span><span style="color: Tomato;">(</span><span style="color: Tomato;">'</span><span style="color: Tomato;">i</span><span style="color: Tomato;">m</span><span style="color: Tomato;">g</span><span style="color: Tomato;">'</span><span style="color: Tomato;">,</span><span style="color: Tomato;"> </span><span style="color: Tomato;">{</span><span style="color: Tomato;">}</span><span style="color: Tomato;">,</span><span style="color: Tomato;"> </span><span style="color: Tomato;">'</span><span style="color: Tomato;">c</span><span style="color: Tomato;">h</span><span style="color: Tomato;">i</span><span style="color: Tomato;">l</span><span style="color: Tomato;">d</span><span style="color: Tomato;">'</span><span style="color: Tomato;">)</span>
    <span style="color: rgb(38, 148, 255);">│</span> <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove the </span><span style="color: rgb(38, 148, 255);"><strong>children</strong></span><span style="color: rgb(38, 148, 255);">.</span>
  
      | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
  0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">React.createElement('img', {}, 'child')</span>
    0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">React.createElement('img', {}, )</span>
  
</code></pre>{% endraw %}

