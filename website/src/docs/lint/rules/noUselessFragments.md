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

{% raw %}<pre class="language-text"><code class="language-text">nursery/noUselessFragments.js:1:1 <a href="https://rome.tools/docs/lint/rules/noUselessFragments">lint/nursery/noUselessFragments</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Avoid using unnecessary </span><span style="color: Orange;"><strong>Fragment</strong></span><span style="color: Orange;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>foo
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>&lt;/&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove the Fragment</span>
  
      | <span style="color: rgb(38, 148, 255);">@@ -1,3 +1,3 @@</span>
  0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">&lt;&gt;</span>
    0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">&quot;</span>
  1 1 |   foo
  2   | <span style="color: Tomato;">- </span><span style="color: Tomato;">&lt;/&gt;</span>
    2 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">&quot;</span>
  
</code></pre>{% endraw %}

```jsx
<React.Fragment>
foo
</React.Fragment>
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noUselessFragments.js:1:1 <a href="https://rome.tools/docs/lint/rules/noUselessFragments">lint/nursery/noUselessFragments</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Avoid using unnecessary </span><span style="color: Orange;"><strong>Fragment</strong></span><span style="color: Orange;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;React.Fragment&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>foo
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>3 │ </strong>&lt;/React.Fragment&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>4 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove the Fragment</span>
  
      | <span style="color: rgb(38, 148, 255);">@@ -1,3 +1,3 @@</span>
  0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">&lt;React.Fragment&gt;</span>
    0 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">&quot;</span>
  1 1 |   foo
  2   | <span style="color: Tomato;">- </span><span style="color: Tomato;">&lt;/React.Fragment&gt;</span>
    2 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">&quot;</span>
  
</code></pre>{% endraw %}

```jsx
<>
    <>foo</>
    <SomeComponent />
</>
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noUselessFragments.js:2:5 <a href="https://rome.tools/docs/lint/rules/noUselessFragments">lint/nursery/noUselessFragments</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Avoid using unnecessary </span><span style="color: Orange;"><strong>Fragment</strong></span><span style="color: Orange;">.</span>
  
    <strong>1 │ </strong>&lt;&gt;
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    &lt;&gt;foo&lt;/&gt;
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>    &lt;SomeComponent /&gt;
    <strong>4 │ </strong>&lt;/&gt;
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove the Fragment</span>
  
      | <span style="color: rgb(38, 148, 255);">@@ -1,4 +1,4 @@</span>
  0 0 |   &lt;&gt;
  1   | <span style="color: Tomato;">- </span><span style="color: Tomato;">    &lt;&gt;foo&lt;/&gt;</span>
    1 | <span style="color: MediumSeaGreen;">+ </span><span style="color: MediumSeaGreen;">    foo</span>
  2 2 |       &lt;SomeComponent /&gt;
  3 3 |   &lt;/&gt;
  
</code></pre>{% endraw %}

```jsx
<></>
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noUselessFragments.js:1:1 <a href="https://rome.tools/docs/lint/rules/noUselessFragments">lint/nursery/noUselessFragments</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Avoid using unnecessary </span><span style="color: Orange;"><strong>Fragment</strong></span><span style="color: Orange;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;&gt;&lt;/&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove the Fragment</span>
  
      | <span style="color: rgb(38, 148, 255);">@@ -1 +1 @@</span>
  0   | <span style="color: Tomato;">- </span><span style="color: Tomato;">&lt;&gt;&lt;/&gt;</span>
    0 | <span style="color: MediumSeaGreen;">+ </span>
  
</code></pre>{% endraw %}

