---
title: Lint Rule noDangerouslySetInnerHtml
layout: layouts/rule.liquid
---

# noDangerouslySetInnerHtml (since v0.10.0)

Prevent the usage of dangerous JSX props

## Examples

### Invalid

```jsx
function createMarkup() {
    return { __html: 'child' }
}
<div dangerouslySetInnerHTML={createMarkup()}></div>
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noDangerouslySetInnerHtml.js:4:6 <a href="https://rome.tools/docs/lint/rules/noDangerouslySetInnerHtml">lint/correctness/noDangerouslySetInnerHtml</a> ━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Avoid passing content using the </span><span style="color: Orange;"><strong>dangerouslySetInnerHTML</strong></span><span style="color: Orange;"> prop.</span>
  
    <strong>2 │ </strong>    return { __html: 'child' }
    <strong>3 │ </strong>}
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>4 │ </strong>&lt;div dangerouslySetInnerHTML={createMarkup()}&gt;&lt;/div&gt;
   <strong>   │ </strong>     <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>5 │ </strong>
  
<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Setting content using code can expose users to cross-site scripting (XSS) attacks</span>
  
</code></pre>{% endraw %}

```jsx
React.createElement('div', {
    dangerouslySetInnerHTML: { __html: 'child' }
});
```

{% raw %}<pre class="language-text"><code class="language-text">correctness/noDangerouslySetInnerHtml.js:2:5 <a href="https://rome.tools/docs/lint/rules/noDangerouslySetInnerHtml">lint/correctness/noDangerouslySetInnerHtml</a> ━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Avoid passing content using the </span><span style="color: Orange;"><strong>dangerouslySetInnerHTML</strong></span><span style="color: Orange;"> prop.</span>
  
    <strong>1 │ </strong>React.createElement('div', {
<strong><span style="color: Tomato;">  </span></strong><strong><span style="color: Tomato;">&gt;</span></strong> <strong>2 │ </strong>    dangerouslySetInnerHTML: { __html: 'child' }
   <strong>   │ </strong>    <strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong><strong><span style="color: Tomato;">^</span></strong>
    <strong>3 │ </strong>});
    <strong>4 │ </strong>
  
<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Setting content using code can expose users to cross-site scripting (XSS) attacks</span>
  
</code></pre>{% endraw %}

