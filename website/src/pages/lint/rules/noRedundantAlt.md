---
title: Lint Rule noRedundantAlt
parent: lint/rules/index
---

# noRedundantAlt (since v12.0.0)

Enforce `img` alt prop does not contain the word "image", "picture", or "photo".
The rule will first check if aria-hidden is true to determine whether to enforce the rule. If the image is
hidden, then rule will always succeed.

## Examples

### Invalid

```jsx
<img src="src" alt="photo content" />;
```

<pre class="language-text"><code class="language-text">nursery/noRedundantAlt.js:1:20 <a href="https://docs.rome.tools/lint/rules/noRedundantAlt">lint/nursery/noRedundantAlt</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid the words &quot;image&quot;, &quot;picture&quot;, or &quot;photo&quot; in</span><span style="color: Tomato;"><strong>img</strong></span><span style="color: Tomato;"> element alt text.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;img src=&quot;src&quot; alt=&quot;photo content&quot; /&gt;;
   <strong>   │ </strong>                   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Screen readers announce img elements as &quot;images&quot;, so it is not necessary to redeclare this in alternative text.</span>
  
</code></pre>

```jsx
<img alt={`picture doing ${things}`} {...this.props} />;
```

<pre class="language-text"><code class="language-text">nursery/noRedundantAlt.js:1:10 <a href="https://docs.rome.tools/lint/rules/noRedundantAlt">lint/nursery/noRedundantAlt</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid the words &quot;image&quot;, &quot;picture&quot;, or &quot;photo&quot; in</span><span style="color: Tomato;"><strong>img</strong></span><span style="color: Tomato;"> element alt text.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;img alt={`picture doing ${things}`} {...this.props} /&gt;;
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Screen readers announce img elements as &quot;images&quot;, so it is not necessary to redeclare this in alternative text.</span>
  
</code></pre>

```jsx
<img alt="picture of cool person" aria-hidden={false} />;
```

<pre class="language-text"><code class="language-text">nursery/noRedundantAlt.js:1:10 <a href="https://docs.rome.tools/lint/rules/noRedundantAlt">lint/nursery/noRedundantAlt</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Avoid the words &quot;image&quot;, &quot;picture&quot;, or &quot;photo&quot; in</span><span style="color: Tomato;"><strong>img</strong></span><span style="color: Tomato;"> element alt text.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;img alt=&quot;picture of cool person&quot; aria-hidden={false} /&gt;;
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Screen readers announce img elements as &quot;images&quot;, so it is not necessary to redeclare this in alternative text.</span>
  
</code></pre>

### Valid

```jsx
<img src="src" alt="alt" />;
<img src="src" alt={photo} />;
<img src="bar" aria-hidden alt="Picture of me taking a photo of an image" />
```

