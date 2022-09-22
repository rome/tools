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

{% raw %}<pre class="language-text"><code class="language-text">nursery/noDangerouslySetInnerHtml.js:4:6 <a href="https://rome.tools/docs/lint/rules/noDangerouslySetInnerHtml">lint/nursery/noDangerouslySetInnerHtml</a> ━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Avoid passing content using the </span><span style="color: Orange;"><strong>dangerouslySetInnerHTML</strong></span><span style="color: Orange;"> prop.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noDangerouslySetInnerHtml.js:4:6
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">4</span> <span style="color: rgb(38, 148, 255);">│</span> &lt;div <span style="color: Tomato;">d</span><span style="color: Tomato;">a</span><span style="color: Tomato;">n</span><span style="color: Tomato;">g</span><span style="color: Tomato;">e</span><span style="color: Tomato;">r</span><span style="color: Tomato;">o</span><span style="color: Tomato;">u</span><span style="color: Tomato;">s</span><span style="color: Tomato;">l</span><span style="color: Tomato;">y</span><span style="color: Tomato;">S</span><span style="color: Tomato;">e</span><span style="color: Tomato;">t</span><span style="color: Tomato;">I</span><span style="color: Tomato;">n</span><span style="color: Tomato;">n</span><span style="color: Tomato;">e</span><span style="color: Tomato;">r</span><span style="color: Tomato;">H</span><span style="color: Tomato;">T</span><span style="color: Tomato;">M</span><span style="color: Tomato;">L</span>={createMarkup()}&gt;&lt;/div&gt;
    <span style="color: rgb(38, 148, 255);">│</span>      <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Setting content using code can expose users to cross-site scripting (XSS) attacks</span>
  
</code></pre>{% endraw %}

```jsx
React.createElement('div', {
    dangerouslySetInnerHTML: { __html: 'child' }
});
```

{% raw %}<pre class="language-text"><code class="language-text">nursery/noDangerouslySetInnerHtml.js:2:5 <a href="https://rome.tools/docs/lint/rules/noDangerouslySetInnerHtml">lint/nursery/noDangerouslySetInnerHtml</a> ━━━━━━━━━━━━━━━━━━━━

<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Avoid passing content using the </span><span style="color: Orange;"><strong>dangerouslySetInnerHTML</strong></span><span style="color: Orange;"> prop.</span>
  
    <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noDangerouslySetInnerHtml.js:2:5
    <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">  </span><span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span>     <span style="color: Tomato;">d</span><span style="color: Tomato;">a</span><span style="color: Tomato;">n</span><span style="color: Tomato;">g</span><span style="color: Tomato;">e</span><span style="color: Tomato;">r</span><span style="color: Tomato;">o</span><span style="color: Tomato;">u</span><span style="color: Tomato;">s</span><span style="color: Tomato;">l</span><span style="color: Tomato;">y</span><span style="color: Tomato;">S</span><span style="color: Tomato;">e</span><span style="color: Tomato;">t</span><span style="color: Tomato;">I</span><span style="color: Tomato;">n</span><span style="color: Tomato;">n</span><span style="color: Tomato;">e</span><span style="color: Tomato;">r</span><span style="color: Tomato;">H</span><span style="color: Tomato;">T</span><span style="color: Tomato;">M</span><span style="color: Tomato;">L</span>: { __html: 'child' }
    <span style="color: rgb(38, 148, 255);">│</span>     <span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span><span style="color: Tomato;">^</span>
  
<strong><span style="color: Orange;">  </span></strong><strong><span style="color: Orange;">⚠</span></strong> <span style="color: Orange;">Setting content using code can expose users to cross-site scripting (XSS) attacks</span>
  
</code></pre>{% endraw %}

