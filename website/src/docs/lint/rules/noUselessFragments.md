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
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>&lt;</strong></span><span style="color: Tomato;"><strong>&gt;</strong></span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>&quot;</strong></span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  foo
    <strong>3</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>&lt;</strong></span><span style="color: Tomato;"><strong>/</strong></span><span style="color: Tomato;"><strong>&gt;</strong></span>
      <strong>3</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>&quot;</strong></span>
    <strong>4</strong> <strong>4</strong><strong> │ </strong>  
  
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
  
    <strong>1</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>&lt;</strong></span><span style="color: Tomato;"><strong>R</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>c</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>.</strong></span><span style="color: Tomato;"><strong>F</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>g</strong></span><span style="color: Tomato;"><strong>m</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>&gt;</strong></span>
      <strong>1</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>&quot;</strong></span>
    <strong>2</strong> <strong>2</strong><strong> │ </strong>  foo
    <strong>3</strong>  <strong> │ </strong><span style="color: Tomato;">-</span> <span style="color: Tomato;"><strong>&lt;</strong></span><span style="color: Tomato;"><strong>/</strong></span><span style="color: Tomato;"><strong>R</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>c</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>.</strong></span><span style="color: Tomato;"><strong>F</strong></span><span style="color: Tomato;"><strong>r</strong></span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"><strong>g</strong></span><span style="color: Tomato;"><strong>m</strong></span><span style="color: Tomato;"><strong>e</strong></span><span style="color: Tomato;"><strong>n</strong></span><span style="color: Tomato;"><strong>t</strong></span><span style="color: Tomato;"><strong>&gt;</strong></span>
      <strong>3</strong><strong> │ </strong><span style="color: MediumSeaGreen;">+</span> <span style="color: MediumSeaGreen;"><strong>&quot;</strong></span>
    <strong>4</strong> <strong>4</strong><strong> │ </strong>  
  
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
  
<strong>  </strong><strong>  2 │ </strong><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="opacity: 0.8;">·</span><span style="color: Tomato;">&lt;</span><span style="color: Tomato;">&gt;</span>foo<span style="color: Tomato;">&lt;</span><span style="color: Tomato;">/</span><span style="color: Tomato;">&gt;</span>
<strong>  </strong><strong>    │ </strong>    <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>   <span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>
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
  
<strong>  </strong><strong>  1 │ </strong><span style="color: Tomato;">&lt;</span><span style="color: Tomato;">&gt;</span><span style="color: Tomato;">&lt;</span><span style="color: Tomato;">/</span><span style="color: Tomato;">&gt;</span>
<strong>  </strong><strong>    │ </strong><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>
</code></pre>{% endraw %}

