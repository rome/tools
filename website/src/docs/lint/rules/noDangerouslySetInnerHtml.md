---
title: Lint Rule noDangerouslySetInnerHtml
layout: layouts/rule.liquid
---

# noDangerouslySetInnerHtml (since v0.10.0)

Prevent the usage of dangerous JSX props

## Examples

### Invalid

```jsx
<div dangerouslySetInnerHTML={{ __html: "child" }}></div>
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noDangerouslySetInnerHtml/">nursery/noDangerouslySetInnerHtml</a></span><span style="color: Orange;">]</span><em>: </em><em>Avoid passing content using the </em><em><em>dangerouslySetInnerHTML</em></em><em> prop.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noDangerouslySetInnerHtml.js:1:6
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">1</span> <span style="color: rgb(38, 148, 255);">│</span> &lt;div dangerouslySetInnerHTML={{ __html: &quot;child&quot; }}&gt;&lt;/div&gt;
  <span style="color: rgb(38, 148, 255);">│</span>      <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

= <span style="color: Orange;">warning</span><span style="color: Orange;">: </span>Setting content using code can expose users to cross-site scripting (XSS) attacks

</code></pre>{% endraw %}

```jsx
React.createElement('div', {
    dangerouslySetInnerHTML: { __html: "child" }
});
```

{% raw %}<pre class="language-text"><code class="language-text"><span style="color: Orange;">warning</span><span style="color: Orange;">[</span><span style="color: Orange;"><a href="https://rome.tools/docs/lint/rules/noDangerouslySetInnerHtml/">nursery/noDangerouslySetInnerHtml</a></span><span style="color: Orange;">]</span><em>: </em><em>Avoid passing content using the </em><em><em>dangerouslySetInnerHTML</em></em><em> prop.</em>
  <span style="color: rgb(38, 148, 255);">┌</span><span style="color: rgb(38, 148, 255);">─</span> nursery/noDangerouslySetInnerHtml.js:2:5
  <span style="color: rgb(38, 148, 255);">│</span>
<span style="color: rgb(38, 148, 255);">2</span> <span style="color: rgb(38, 148, 255);">│</span>     dangerouslySetInnerHTML: { __html: &quot;child&quot; }
  <span style="color: rgb(38, 148, 255);">│</span>     <span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span><span style="color: rgb(38, 148, 255);">-</span>

= <span style="color: Orange;">warning</span><span style="color: Orange;">: </span>Setting content using code can expose users to cross-site scripting (XSS) attacks

</code></pre>{% endraw %}

