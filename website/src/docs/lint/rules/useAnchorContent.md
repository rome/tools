---
title: Lint Rule useAnchorContent
layout: layouts/rule.liquid
---

# useAnchorContent (since v10.0.0)

> This rule is recommended by Rome.

Enforce that anchor elements have content and that the content is accessible to screen readers.

Accessible means that the content is not hidden using the `aria-hidden` attribute.

## Examples

### Invalid

```jsx
<a />
```

{% raw %}<pre class="language-text"><code class="language-text">a11y/useAnchorContent.js:1:1 <a href="https://rome.tools/docs/lint/rules/useAnchorContent">lint/a11y/useAnchorContent</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide screen reader accessible content when using </span><span style="color: Tomato;"><strong>`a`</strong></span><span style="color: Tomato;"> elements.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;a /&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">All links on a page should have content that is accessible to screen readers.</span>
  
</code></pre>{% endraw %}

```jsx
<a></a>
```

{% raw %}<pre class="language-text"><code class="language-text">a11y/useAnchorContent.js:1:1 <a href="https://rome.tools/docs/lint/rules/useAnchorContent">lint/a11y/useAnchorContent</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide screen reader accessible content when using </span><span style="color: Tomato;"><strong>`a`</strong></span><span style="color: Tomato;"> elements.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;a&gt;&lt;/a&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">All links on a page should have content that is accessible to screen readers.</span>
  
</code></pre>{% endraw %}

```jsx
<a>    </a>
```

{% raw %}<pre class="language-text"><code class="language-text">a11y/useAnchorContent.js:1:1 <a href="https://rome.tools/docs/lint/rules/useAnchorContent">lint/a11y/useAnchorContent</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide screen reader accessible content when using </span><span style="color: Tomato;"><strong>`a`</strong></span><span style="color: Tomato;"> elements.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;a&gt;    &lt;/a&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">All links on a page should have content that is accessible to screen readers.</span>
  
</code></pre>{% endraw %}

```jsx
<a aria-hidden>content</a>
```

{% raw %}<pre class="language-text"><code class="language-text">a11y/useAnchorContent.js:1:1 <a href="https://rome.tools/docs/lint/rules/useAnchorContent">lint/a11y/useAnchorContent</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide screen reader accessible content when using </span><span style="color: Tomato;"><strong>`a`</strong></span><span style="color: Tomato;"> elements.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;a aria-hidden&gt;content&lt;/a&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">All links on a page should have content that is accessible to screen readers.</span>
  
</code></pre>{% endraw %}

```jsx
<a><span aria-hidden="true">content</span></a
```

{% raw %}<pre class="language-text"><code class="language-text">a11y/useAnchorContent.js:2:1 parse ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">expected `&gt;` but instead the file ends</span>
  
    <strong>1 │ </strong>&lt;a&gt;&lt;span aria-hidden=&quot;true&quot;&gt;content&lt;/span&gt;&lt;/a
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>
   <strong>   │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">the file ends here</span>
  
    <strong>1 │ </strong>&lt;a&gt;&lt;span aria-hidden=&quot;true&quot;&gt;content&lt;/span&gt;&lt;/a
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>
   <strong>   │ </strong>
  
</code></pre>{% endraw %}

## Valid

```jsx
<a>content</a>
```

```jsx
function html() {
    return { __html: "foo" }
}
<a dangerouslySetInnerHTML={html()} />
```

```jsx
<a><TextWrapper aria-hidden={true} /> content</a>
```

```jsx
<a><div aria-hidden="true"></div> content</a>
```

