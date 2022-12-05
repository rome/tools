---
title: Lint Rule noDistractingElements
parent: lint/rules/index
---

# noDistractingElements (since v11.0.0)

Enforces that no distracting elements are used.

Elements that can be visually distracting can cause accessibility issues with visually impaired users.
Such elements are most likely deprecated, and should be avoided.
By default, the following elements are visually distracting: `<marquee>` and `<blink>`.

## Examples

### Invalid

```jsx
<marquee/>
```

<pre class="language-text"><code class="language-text">nursery/noDistractingElements.js:1:1 <a href="https://docs.rome.tools/lint/rules/noDistractingElements">lint/nursery/noDistractingElements</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Don't use the 'marquee' element.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;marquee/&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Visually distracting elements can cause accessibility issues and should be avoided.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove the 'marquee' element.</span>
  
<strong>  </strong><strong>  1 │ </strong><span style="color: Tomato;">&lt;</span><span style="color: Tomato;">m</span><span style="color: Tomato;">a</span><span style="color: Tomato;">r</span><span style="color: Tomato;">q</span><span style="color: Tomato;">u</span><span style="color: Tomato;">e</span><span style="color: Tomato;">e</span><span style="color: Tomato;">/</span><span style="color: Tomato;">&gt;</span>
<strong>  </strong><strong>    │ </strong><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>
</code></pre>

```jsx
<blink/>
```

<pre class="language-text"><code class="language-text">nursery/noDistractingElements.js:1:1 <a href="https://docs.rome.tools/lint/rules/noDistractingElements">lint/nursery/noDistractingElements</a> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Don't use the 'blink' element.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;blink/&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Visually distracting elements can cause accessibility issues and should be avoided.</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Suggested fix</span><span style="color: rgb(38, 148, 255);">: </span><span style="color: rgb(38, 148, 255);">Remove the 'blink' element.</span>
  
<strong>  </strong><strong>  1 │ </strong><span style="color: Tomato;">&lt;</span><span style="color: Tomato;">b</span><span style="color: Tomato;">l</span><span style="color: Tomato;">i</span><span style="color: Tomato;">n</span><span style="color: Tomato;">k</span><span style="color: Tomato;">/</span><span style="color: Tomato;">&gt;</span>
<strong>  </strong><strong>    │ </strong><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span><span style="color: Tomato;">-</span>
</code></pre>

### Valid

```jsx
<div/>
```

