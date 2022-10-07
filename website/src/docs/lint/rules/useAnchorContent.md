---
title: Lint Rule useAnchorContent
layout: layouts/rule.liquid
---

# useAnchorContent (since v10.0.0)

Enforce that anchor elements have content and that the content is accessible to screen readers.

Accessible means that the content is not hidden using the `aria-hidden` attribute.

## Examples

### Invalid

```jsx
<a />
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/useAnchorContent.js:1:1 <a href="https://rome.tools/docs/lint/rules/useAnchorContent">lint/nursery/useAnchorContent</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Provide screen reader accessible content when using </span><span style="color: Orange;"><strong>anchor</strong></span><span style="color: Orange;"> elements.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;a /&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">All links on a page should have content that is accessible to screen readers.</span>
  
</code></pre>{% endraw %}

```jsx
<a><TextWrapper aria-hidden /></a>
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/useAnchorContent.js:1:1 <a href="https://rome.tools/docs/lint/rules/useAnchorContent">lint/nursery/useAnchorContent</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Provide screen reader accessible content when using </span><span style="color: Orange;"><strong>anchor</strong></span><span style="color: Orange;"> elements.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;a&gt;&lt;TextWrapper aria-hidden /&gt;&lt;/a&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">All links on a page should have content that is accessible to screen readers.</span>
  
</code></pre>{% endraw %}

```jsx
<a></a>
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/useAnchorContent.js:1:1 <a href="https://rome.tools/docs/lint/rules/useAnchorContent">lint/nursery/useAnchorContent</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Provide screen reader accessible content when using </span><span style="color: Orange;"><strong>anchor</strong></span><span style="color: Orange;"> elements.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;a&gt;&lt;/a&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">All links on a page should have content that is accessible to screen readers.</span>
  
</code></pre>{% endraw %}

## Valid

```jsx
<a>Anchor Content!</a>
```

```jsx
<a dangerouslySetInnerHTML={{ __html: "foo" }} />
```

```jsx
<a><TextWrapper aria-hidden={true} /> visible content</a>
```

```jsx
<a><div aria-hidden="true"></div>visible content</a>
```

