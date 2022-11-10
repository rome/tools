---
title: Lint Rule useValidAnchor
---

# useValidAnchor (since v10.0.0)

> This rule is recommended by Rome.

Enforce that all anchors are valid, and they are navigable elements.

The anchor element (`<a></a>`) - also called **hyperlink** - is an important element
that allows users to navigate pages, in the same page, same website or on another website.

While before it was possible to attach logic to an anchor element, with the advent of JSX libraries,
it's now  easier to attach logic to any HTML element, anchors included.

This rule is designed to prevent users to attach logic at the click of anchors, and also makes
sure that the `href` provided to the anchor element is valid. If the anchor has logic attached to it,
the rules suggests to turn it to a `button`, because that's likely what the user wants.

Anchor `<a></a>` elements should be used for navigation, while `<button></button>` should be
used for user interaction.

There are **many reasons** why an anchor should not have a logic and have a correct `href` attribute:

- it can disrupt the correct flow of the user navigation e.g. a user that wants to open the link
in another tab, but the default "click" behaviour is prevented;
- it can source of invalid links, and [crawlers](https://en.wikipedia.org/wiki/Web_crawler) can't navigate the website, risking to penalise
[SEO](https://en.wikipedia.org/wiki/Search_engine_optimization) ranking

## Examples

### Invalid

```jsx
<a href={null}>navigate here</a>
```

{% raw %}<pre class="language-text"><code class="language-text">a11y/useValidAnchor.js:1:10 <a href="https://docs.rome.tools/lint/rules/useValidAnchor">lint/a11y/useValidAnchor</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide a valid value for the attribute </span><span style="color: Tomato;"><strong>href</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;a href={null}&gt;navigate here&lt;/a&gt;
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The href attribute should be a valid a URL</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Check </span><span style="color: rgb(38, 148, 255);"><a href="https://marcysutton.com/links-vs-buttons-in-modern-web-applications">this thorough explanation</a></span><span style="color: rgb(38, 148, 255);"> to better understand the context.</span>
  
</code></pre>{% endraw %}

```jsx
<a href={undefined}>navigate here</a>
```

{% raw %}<pre class="language-text"><code class="language-text">a11y/useValidAnchor.js:1:10 <a href="https://docs.rome.tools/lint/rules/useValidAnchor">lint/a11y/useValidAnchor</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide a valid value for the attribute </span><span style="color: Tomato;"><strong>href</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;a href={undefined}&gt;navigate here&lt;/a&gt;
   <strong>   │ </strong>         <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The href attribute should be a valid a URL</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Check </span><span style="color: rgb(38, 148, 255);"><a href="https://marcysutton.com/links-vs-buttons-in-modern-web-applications">this thorough explanation</a></span><span style="color: rgb(38, 148, 255);"> to better understand the context.</span>
  
</code></pre>{% endraw %}

```jsx
<a href>navigate here</a>
```

{% raw %}<pre class="language-text"><code class="language-text">a11y/useValidAnchor.js:1:4 <a href="https://docs.rome.tools/lint/rules/useValidAnchor">lint/a11y/useValidAnchor</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">The attribute </span><span style="color: Tomato;"><strong>href</strong></span><span style="color: Tomato;"> has to be assigned to a valid value.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;a href&gt;navigate here&lt;/a&gt;
   <strong>   │ </strong>   <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The href attribute should be a valid a URL</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Check </span><span style="color: rgb(38, 148, 255);"><a href="https://marcysutton.com/links-vs-buttons-in-modern-web-applications">this thorough explanation</a></span><span style="color: rgb(38, 148, 255);"> to better understand the context.</span>
  
</code></pre>{% endraw %}

```jsx
<a href="javascript:void(0)">navigate here</a>
```

{% raw %}<pre class="language-text"><code class="language-text">a11y/useValidAnchor.js:1:9 <a href="https://docs.rome.tools/lint/rules/useValidAnchor">lint/a11y/useValidAnchor</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Provide a valid value for the attribute </span><span style="color: Tomato;"><strong>href</strong></span><span style="color: Tomato;">.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;a href=&quot;javascript:void(0)&quot;&gt;navigate here&lt;/a&gt;
   <strong>   │ </strong>        <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">The href attribute should be a valid a URL</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Check </span><span style="color: rgb(38, 148, 255);"><a href="https://marcysutton.com/links-vs-buttons-in-modern-web-applications">this thorough explanation</a></span><span style="color: rgb(38, 148, 255);"> to better understand the context.</span>
  
</code></pre>{% endraw %}

```jsx
<a href="https://example.com" onClick={something}>navigate here</a>
```

{% raw %}<pre class="language-text"><code class="language-text">a11y/useValidAnchor.js:1:1 <a href="https://docs.rome.tools/lint/rules/useValidAnchor">lint/a11y/useValidAnchor</a> ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">✖</span></strong> <span style="color: Tomato;">Use a </span><span style="color: Tomato;"><strong>button</strong></span><span style="color: Tomato;"> element instead of an </span><span style="color: Tomato;"><strong>a</strong></span><span style="color: Tomato;"> element.</span>
  
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>1 │ </strong>&lt;a href=&quot;https://example.com&quot; onClick={something}&gt;navigate here&lt;/a&gt;
   <strong>   │ </strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>2 │ </strong>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Anchor elements should only be used for default sections or page navigation</span>
  
<strong><span style="color: rgb(38, 148, 255);">  </span></strong><strong><span style="color: rgb(38, 148, 255);">ℹ</span></strong> <span style="color: rgb(38, 148, 255);">Check </span><span style="color: rgb(38, 148, 255);"><a href="https://marcysutton.com/links-vs-buttons-in-modern-web-applications">this thorough explanation</a></span><span style="color: rgb(38, 148, 255);"> to better understand the context.</span>
  
</code></pre>{% endraw %}

### Valid

```jsx
<>
    <a href={`https://www.javascript.com`}>navigate here</a>
    <a href={somewhere}>navigate here</a>
    <a {...spread}>navigate here</a>
</>
```

## Accessibility guidelines

[WCAG 2.1.1](https://www.w3.org/WAI/WCAG21/Understanding/keyboard)

## Resources

- [WebAIM - Introduction to Links and Hypertext](https://webaim.org/techniques/hypertext/)
- [Links vs. Buttons in Modern Web Applications](https://marcysutton.com/links-vs-buttons-in-modern-web-applications/)
- [Using ARIA - Notes on ARIA use in HTML](https://www.w3.org/TR/using-aria/#NOTES)

